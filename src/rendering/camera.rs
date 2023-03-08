use std::{thread::{Thread, self}, sync::Arc};

use rand::{rngs::ThreadRng, thread_rng};

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

    pub fn get_image_threaded(&self, render_config: &RenderConfig, is_screenshot: bool) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0; (render_config.resolution.0 * render_config.resolution.1 * 3) as usize];
        
        let chunks = data.chunks_mut(((render_config.resolution.0 * render_config.resolution.1 * 3) / 12) as usize);

        // let mut offset = 0;
        // for chunk in chunks {
        //     ts.push(thread::scope(|scope| { Self::get_image_sub_threaded(self.clone(), render_config.clone(), is_screenshot, chunk, offset); }));
        //     offset += chunk.len();
        // }

        thread::scope(|scope| {
            let mut ts = Vec::with_capacity(chunks.len());

            let mut offset = 0;
            for chunk in chunks {
                let o = offset;
                let len = chunk.len();
                ts.push(scope.spawn(move || { Self::get_image_sub_threaded(self.clone(), render_config.clone(), is_screenshot, chunk, o); }));
                offset += len;
            }

            for t in ts {
                t.join().unwrap();
            }
        });

        data
    }

    pub fn get_image_sub_threaded(cam: Camera, render_config: &RenderConfig, is_screenshot: bool, chunk: &mut [u8], offset: usize) {
        let resolution;
        if is_screenshot { resolution = render_config.screenshot_resolution }
        else { resolution = render_config.resolution; }

        let mut rng = thread_rng();

        for i in 0..(chunk.len() / 3) {
            let x = (i + offset) % resolution.0 as usize;
            let y = (i + offset) / resolution.0 as usize;

            let ray_vector = (V3::FORWARD * cam.fov) + 
                V3::new(0.0, 
                    -(((y as i32) - ((resolution.1)/2) as i32) as f64) / ((resolution.1 as f64) / (2.0)), 
                    (((x as i32) - ((resolution.0)/2) as i32) as f64) / ((resolution.0 as f64) / ((resolution.0 as f64 / resolution.1 as f64) * 2.0))
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
                        -(((y as i32) - ((resolution.1)/2) as i32) as f64) / ((resolution.1 as f64) / (2.0)), 
                        (((x as i32) - ((resolution.0)/2) as i32) as f64) / ((resolution.0 as f64) / ((resolution.0 as f64 / resolution.1 as f64) * 2.0))
                    );

                let ray_vector = ray_vector.rotate_z(&V3::ZERO, self.rotation.1);
                let ray_vector = ray_vector.rotate_y(&V3::ZERO, self.rotation.0);

                let ray = Line::new(&self.position,
                    &ray_vector);

                let colour = super::get_colour(ray, render_config, rng, is_screenshot).as_u8();
        
                data.push(colour.0);
                data.push(colour.1);
                data.push(colour.2);
            }
        }

        data
    }
}