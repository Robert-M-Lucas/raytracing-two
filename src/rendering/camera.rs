use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use rand::{rngs::ThreadRng, thread_rng};
use rayon::prelude::*;
use thread_local::ThreadLocal;

use crate::maths::{vectors::V3, lines::Line};

use super::RenderConfig;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Camera {
    pub position: V3,
    pub rotation: (f64, f64), // Y, Z
    pub fov: f64,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(position: &V3, rotation: (f64, f64), fov: f64) -> Self {
        Self { position: position.clone(), rotation, fov }
    }

    pub fn get_image_threaded_old(&self, render_config: &RenderConfig) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0; (render_config.screenshot_resolution.0 * render_config.screenshot_resolution.1 * 3) as usize];

        // T1: 754.2866773s
        // T2: 626.2710919s
        let chunks = data.chunks_mut(
            ((render_config.screenshot_resolution.0 * render_config.screenshot_resolution.1 * 3) as usize) / 
            (num_cpus::get() * 4)
        );
        println!("CPUs available: {}, Threads: {}", num_cpus::get(), chunks.len());

        thread::scope(|scope| {
            let mut ts = Vec::with_capacity(chunks.len());

            let mut offset = 0;
            let mut i: u32 = 0;
            for chunk in chunks {
                let o = offset;
                let len = chunk.len() / 3;
                ts.push(scope.spawn(move || { 
                    Self::get_image_sub_threaded(self.clone(), render_config.clone(), true, chunk, o, i); 
                }));
                offset += len;
                i += 1;
            }

            for t in ts {
                t.join().unwrap();
            }
        });

        data
    }

    pub fn get_image_threaded(&self, render_config: &RenderConfig) -> Vec<u8> {
        let resolution;
        let is_screenshot = true;
        if is_screenshot { resolution = render_config.screenshot_resolution }
        else { resolution = render_config.resolution; }

        thread_local!(static STORE: RefCell<Option<ThreadRng>> = RefCell::new(None));

        const PIXEL_GROUP: usize = 100;

        let data: Vec<Vec<(u8, u8, u8)>> = (0..(resolution.0 * resolution.1)).into_par_iter().chunks(PIXEL_GROUP).map(
            |is| {

            STORE.with(|cell| {
                let mut local_store = cell.borrow_mut();
                if local_store.is_none() {
                    *local_store = Some(thread_rng());
                }

                let rng = local_store.as_mut().unwrap();

                let mut result = Vec::with_capacity(PIXEL_GROUP);
                for i in is {
                    let x = i % resolution.0;
                    let y = i / resolution.0;

                    let ray_vector = (V3::FORWARD * self.fov) +
                        V3::new(0.0,
                                -(((y as i32) - ((resolution.1) / 2) as i32) as f64) /
                                    ((resolution.1 as f64) / (2.0)),
                                (((x as i32) - ((resolution.0) / 2) as i32) as f64) /
                                    ((resolution.0 as f64) / ((resolution.0 as f64 / resolution.1 as f64) * 2.0))
                        );

                    let ray_vector = ray_vector.rotate_z(&V3::ZERO, self.rotation.1);
                    let ray_vector = ray_vector.rotate_y(&V3::ZERO, self.rotation.0);

                    let ray = Line::new(&self.position, &ray_vector);

                    let colour = super::get_colour(ray, render_config, rng, is_screenshot).as_u8();

                    result.push((colour.0, colour.1, colour.2));
                }

                result
            })
        }
        ).collect();

        let mut flattened_data = Vec::with_capacity((resolution.0 * resolution.1 * 3) as usize);

        for result in data {
            for c in result {
                flattened_data.push(c.0);
                flattened_data.push(c.1);
                flattened_data.push(c.2);
            }
        }

        flattened_data
    }

    //noinspection DuplicatedCode
    pub fn get_image_sub_threaded(cam: Camera, render_config: &RenderConfig, is_screenshot: bool, chunk: &mut [u8], offset: usize, thread_id: u32) {
        let resolution;
        if is_screenshot { resolution = render_config.screenshot_resolution }
        else { resolution = render_config.resolution; }

        let mut rng = thread_rng();

        println!("Thread[{}] start", thread_id);
        let mut progress: usize = 0;
        let increment: usize = (chunk.len() / 3) as usize / 10;

        for i in 0..(chunk.len() / 3) {
            if (i / increment) != progress {
                progress = i / increment;
                println!("Thread[{}]: {}%", thread_id, progress * 10);
            }

            let x = (i + offset) % resolution.0 as usize;
            let y = (i + offset) / resolution.0 as usize;

            let ray_vector = (V3::FORWARD * cam.fov) + 
                V3::new(0.0, 
                    -(((y as i32) - ((resolution.1)/2) as i32) as f64) / 
                        ((resolution.1 as f64) / (2.0)), 
                    (((x as i32) - ((resolution.0)/2) as i32) as f64) / 
                        ((resolution.0 as f64) / ((resolution.0 as f64 / resolution.1 as f64) * 2.0))
                );

            let ray_vector = ray_vector.rotate_z(&V3::ZERO, cam.rotation.1);
            let ray_vector = ray_vector.rotate_y(&V3::ZERO, cam.rotation.0);

            let ray = Line::new(&cam.position,
                &ray_vector);

            let colour = super::get_colour(ray, render_config, &mut rng, is_screenshot).as_u8();
    
            chunk[i*3] = colour.0;
            chunk[i*3 + 1] = colour.1;
            chunk[i*3 + 2] = colour.2;
        }

        println!("Thread[{}] complete", thread_id)

        // data
    }

    pub fn get_image(&self, render_config: &RenderConfig, rng: &mut ThreadRng, is_screenshot: bool, verbose: bool) -> Vec<u8> {
        let resolution;
        if is_screenshot { resolution = render_config.screenshot_resolution }
        else { resolution = render_config.resolution; }

        let mut data = Vec::with_capacity((resolution.0 * resolution.1 * 3) as usize);
        
        let mut progress: u32 = 0;
        let increment: u32 = (resolution.0 * resolution.1) / 100;

        if verbose { println!("0%"); }

        for y in 0..resolution.1 {
            for x in 0..resolution.0 {
                if verbose && ((resolution.0 * y + x) / increment) != progress {
                    progress = (resolution.0 * y + x) / increment;
                    println!("{}%", progress);
                }

                let ray_vector = (V3::FORWARD * self.fov) + 
                    V3::new(0.0, 
                        -(((y as i32) - ((resolution.1)/2) as i32) as f64) / 
                            ((resolution.1 as f64) / (2.0)), 
                        (((x as i32) - ((resolution.0)/2) as i32) as f64) / 
                            ((resolution.0 as f64) / ((resolution.0 as f64 / resolution.1 as f64) * 2.0))
                    );

                let ray_vector = ray_vector.rotate_z(&V3::ZERO, self.rotation.1);
                let ray_vector = ray_vector.rotate_y(&V3::ZERO, self.rotation.0);

                let ray = Line::new(&self.position, &ray_vector);

                let colour = super::get_colour(ray, render_config, rng, is_screenshot).as_u8();
        
                data.push(colour.0);
                data.push(colour.1);
                data.push(colour.2);
            }
        }

        data
    }
}