use rand::rngs::ThreadRng;

use crate::maths::{vectors::V3, lines::Line};

use super::RenderConfig;

#[allow(dead_code)]
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