pub mod camera;
use std::fs;

pub use camera::Camera;
use chrono::{Datelike, Timelike};
use image::{DynamicImage, ImageBuffer, Rgb};
use crate::{maths::{Line, Intersection, vectors::V3}, colour::Colour};
pub mod renderer;
pub mod render_config;
pub use render_config::RenderConfig;

pub fn take_screenshot(camera: &Camera, render_config: &RenderConfig) {
    println!("Rendering screenshot...");
    let pixel_data = camera.get_image(render_config, true);
    println!("Saving screenshot...");
    
    let img = DynamicImage::ImageRgb8(ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(
        render_config.resolution.0, 
        render_config.resolution.1, 
        pixel_data
    ).unwrap());

    let now = chrono::offset::Local::now();
    
    let file_name = format!("Render {}-{}-{} {}-{}-{} {}x{}", 
        format!("{:0>2}", now.year()), format!("{:0>2}", now.month()), 
        format!("{:0>2}", now.day()), format!("{:0>2}", now.hour()), 
        format!("{:0>2}", now.minute()), format!("{:0>2}", now.second()), render_config.resolution.0, render_config.resolution.1);
        
    let mut is_err = fs::create_dir_all("renders").is_err();
    if !is_err {
        is_err = img.save(format!("renders\\{}.png", file_name)).is_err();
    }
    if is_err { println!("Saving file to 'renders\\{}.png' failed!", file_name); }
    else { println!("Saved file to 'renders\\{}.png'", file_name); }
}

pub fn get_colour(ray: Line, render_config: &RenderConfig, is_screenshot: bool) -> Colour {
    let max_reflections;
    if is_screenshot { max_reflections = render_config.screenshot_max_reflection; }
    else { max_reflections = render_config.max_reflections; }
    get_colour_recursively(ray, render_config, max_reflections)
}

fn get_colour_recursively(ray: Line, render_config: &RenderConfig, reflection_depth_remaining: u32) -> Colour {
    let mut closest_dist = f64::INFINITY;
    let mut closest_object = None;
    let mut closest_hit = None;

    for i in 0..render_config.scene_objects.len() {
        match Intersection::closest_bounded(&render_config.scene_objects[i].get_intersections(&ray), 0.000001, f64::INFINITY) {
            None => {},
            Some(hit) => { 
                if hit.scale <= closest_dist { 
                    closest_dist = hit.scale;
                    closest_hit = Some(hit.clone());
                    closest_object = Some(&render_config.scene_objects[i]);
                }
            }
        }
    }

    return if !closest_hit.is_some() {
        let offset;
        if ray.vector.y == 0.0 { offset = 0.00001; } else { offset = 0.0; }

        let bg_pos = ray.point + ((&ray.vector + V3::new(0.0, offset, 0.0)) * ((render_config.sky_height - ray.point.y) / (ray.vector.y + offset)));

        render_config.sky_texture.get_colour((bg_pos.x / render_config.sky_scale, bg_pos.z / render_config.sky_scale)).clone()
    } else {
        let closest_hit = closest_hit.unwrap();
        let scene_object = closest_object.unwrap();

        if reflection_depth_remaining == 0 {
            return scene_object.get_colour(&closest_hit).clone();
        }

        let mut new_colour = Colour::BLACK;
        let object_surface_properties = scene_object.get_surface_type();

        if object_surface_properties.opaqueness != 0.0 { 
            new_colour = new_colour + scene_object.get_colour(&closest_hit) * object_surface_properties.opaqueness;
        }
        if object_surface_properties.reflectiveness != 0.0 { 
            new_colour = new_colour +
            get_colour_recursively(scene_object.get_reflection_line(&ray, &closest_hit), 
                render_config, 
                reflection_depth_remaining - 1
            ) * object_surface_properties.reflectiveness;
        }
        if object_surface_properties.transparency != 0.0 { 
            new_colour = new_colour + 
            get_colour_recursively(scene_object.get_transparent_line(&ray, &closest_hit), 
                render_config, 
                reflection_depth_remaining - 1
            ) * object_surface_properties.transparency;
        }

        new_colour.clone()
    }
}