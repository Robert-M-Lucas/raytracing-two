pub mod camera;
use std::{fs, f64::consts::PI};

pub use camera::Camera;
use chrono::{Datelike, Timelike};
use image::{DynamicImage, ImageBuffer, Rgb};
use rand::{thread_rng, Rng, rngs::ThreadRng};
use crate::{maths::{lines::Line, Intersection, vectors::V3}, colour::{Colour}, objects::Object};
pub mod render_config;
pub use render_config::RenderConfig;

pub fn take_screenshot(camera: &Camera, render_config: &RenderConfig, _rng: &mut ThreadRng) {
    println!("Rendering screenshot...");
    
    let pixel_data = camera.get_image_threaded(render_config, true);
    println!("Saving screenshot...");
    
    let img = DynamicImage::ImageRgb8(ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(
        render_config.screenshot_resolution.0, 
        render_config.screenshot_resolution.1, 
        pixel_data
    ).unwrap());

    let now = chrono::offset::Local::now();
    
    let file_name = format!("Render {}-{}-{} {}-{}-{} {}x{}", 
        format!("{:0>2}", now.year()), format!("{:0>2}", now.month()), 
        format!("{:0>2}", now.day()), format!("{:0>2}", now.hour()), 
        format!("{:0>2}", now.minute()), format!("{:0>2}", now.second()), render_config.screenshot_resolution.0, render_config.screenshot_resolution.1);
        
    let mut is_err = fs::create_dir_all("renders").is_err();
    if !is_err {
        is_err = img.save(format!("renders\\{}.png", file_name)).is_err();
    }
    if is_err { println!("Saving file to 'renders\\{}.png' failed!", file_name); }
    else { println!("Saved file to 'renders\\{}.png'", file_name); }
}

pub fn get_colour(ray: Line, render_config: &RenderConfig, rng: &mut ThreadRng, is_screenshot: bool) -> Colour {
    let max_reflections;
    if is_screenshot { max_reflections = render_config.screenshot_max_reflection; }
    else { max_reflections = render_config.max_reflections; }
    get_colour_recursively(ray, render_config, is_screenshot, max_reflections, rng)
}

fn get_direct_light(render_config: &RenderConfig, hit: &Intersection, scene_object: &Box<dyn Object + Sync>, hit_index: usize, rng: &mut ThreadRng) -> Colour {
    let normal = scene_object.get_normal(hit);

    let mut total_colour = Colour::BLACK;

    'light_loop: for l in &render_config.scene_lights {
        let ray = l.get_direct_ray(&hit.position);
        let angle_between = (normal.angle_to(&ray.line.vector) - PI).abs();

        const SMOOTHING_REGION: f64 = 0.2;

        let angle_multiplier;
        if angle_between > PI / 2.0 {
            continue; // Light ray over 90d from hit normal
        }
        // else if angle_between > (PI / 2.0) * (1.0 - SMOOTHING_REGION)   {
        //     let theta = angle_between - ((PI / 2.0) * (1.0 - SMOOTHING_REGION));
        //     angle_multiplier = (1.0 - (theta / ((PI / 2.0) * SMOOTHING_REGION)));
        // }
        // else {
        //     angle_multiplier = 1.0;
        // }

        angle_multiplier = 1.0 - (angle_between / (PI / 2.0));


        for i in 0..render_config.scene_objects.len() {
            if i == hit_index || !render_config.scene_objects[i].get_surface_type().blocks_light { continue; }

            if Intersection::closest_bounded(
                &render_config.scene_objects[i].get_intersections(&ray.line), 
                0.0, ray.scale).is_some() {
                continue 'light_loop;
            }
        }

        total_colour = total_colour + (l.get_colour() * l.get_intensity(ray.length()) * angle_multiplier);
    }

    total_colour
}

#[allow(dead_code)]
fn general_light_falloff(distance: f64) -> f64 {
    1.0 / (distance.powi(2) * 4.0 * PI)
}

fn get_indirect_light(render_config: &RenderConfig, hit: &Intersection, scene_object: &Box<dyn Object + Sync>, hit_index: usize, ray_count: u32, rng: &mut ThreadRng) -> Colour {
    let normal = scene_object.get_normal(hit);

    let mut total_colour = Colour::BLACK;

    for _ in 0..ray_count {
        let ray = Line::new(&hit.position, &V3::get_random(&normal, 0.5 * PI, rng));

        let mut closest_dist = f64::INFINITY;
        let mut closest_object = None;
        let mut closest_hit = None;
        let mut closest_hit_index = None;

        for i in 0..render_config.scene_objects.len() {
            if i == hit_index || !render_config.scene_objects[i].get_surface_type().blocks_light { continue; }

            match Intersection::closest_bounded(&render_config.scene_objects[i].get_intersections(&ray), 0.0, f64::INFINITY) {
                None => {},
                Some(hit) => {
                    if hit.sized_line.scale <= closest_dist { 
                        closest_dist = hit.sized_line.scale;
                        closest_hit = Some(hit.clone());
                        closest_object = Some(&render_config.scene_objects[i]);
                        closest_hit_index = Some(i);
                    }
                }
            }
        }

        if closest_hit.is_none() { continue; }

        let closest_hit = closest_hit.unwrap();
        let closest_object = closest_object.unwrap();
        let surface_type = closest_object.get_surface_type();

        if surface_type.opaqueness == 0.0 || surface_type.diffusiveness == 0.0 { continue; }

        // TODO: Light falloff between 'hit' and 'closest hit'
        let current_colour = 
            get_direct_light(render_config, &closest_hit, &render_config.scene_objects[closest_hit_index.unwrap()], closest_hit_index.unwrap(), rng);

        total_colour = total_colour + (closest_object.get_colour(&closest_hit) * current_colour * surface_type.diffusiveness * surface_type.opaqueness);
    }

    total_colour / (ray_count as f64)
}

fn get_colour_recursively(ray: Line, render_config: &RenderConfig, 
    is_screenshot: bool, reflection_depth_remaining: u32, rng: &mut ThreadRng) -> Colour {
    let mut closest_dist = f64::INFINITY;
    let mut closest_object = None;
    let mut closest_hit = None;
    let mut closest_hit_index = None;

    for i in 0..render_config.scene_objects.len() {
        match Intersection::closest_bounded(&render_config.scene_objects[i].get_intersections(&ray), 0.000001, f64::INFINITY) {
            None => {},
            Some(hit) => { 
                if hit.sized_line.scale <= closest_dist { 
                    closest_dist = hit.sized_line.scale;
                    closest_hit = Some(hit.clone());
                    closest_object = Some(&render_config.scene_objects[i]);
                    closest_hit_index = Some(i);
                }
            }
        }
    }

    return if !closest_hit.is_some() {
        let offset;
        if ray.vector.y == 0.0 { offset = 0.00001; } else { offset = 0.0; }

        let bg_pos = ray.point + ((&ray.vector + V3::new(0.0, offset, 0.0)) * ((render_config.sky_height - ray.point.y) / (ray.vector.y + offset)));

        render_config.sky_texture.get_colour((bg_pos.x / render_config.sky_scale, bg_pos.z / render_config.sky_scale)).clone()
    } 
    else {
        let closest_hit = closest_hit.unwrap();
        let scene_object = closest_object.unwrap();

        if reflection_depth_remaining == 0 {
            return scene_object.get_colour(&closest_hit).clone();
        }

        let mut new_colour = Colour::BLACK;
        let object_surface_properties = scene_object.get_surface_type();

        if object_surface_properties.opaqueness != 0.0 {
            let mut light_colour = Colour::WHITE;

            if !object_surface_properties.full_bright && ((!render_config.enable_full_bright && !is_screenshot) || 
                (!render_config.screenshot_enable_full_bright && is_screenshot)) {
                light_colour = Colour::BLACK;

                if (render_config.enable_direct_lighting && !is_screenshot) || 
                    (render_config.screenshot_enable_direct_lighting && is_screenshot) {
                        light_colour = light_colour + get_direct_light(render_config, &closest_hit, scene_object, closest_hit_index.unwrap(), rng);
                }

                if object_surface_properties.diffusiveness != 0.0 {
                    if render_config.indirect_lighting_ray_count != 0 && !is_screenshot {
                        light_colour = light_colour + 
                            (get_indirect_light(render_config, &closest_hit, scene_object, closest_hit_index.unwrap(), render_config.indirect_lighting_ray_count, rng)
                            * object_surface_properties.diffusiveness);
                    }
                    else if render_config.screenshot_indirect_lighting_ray_count != 0 && is_screenshot {
                        light_colour = light_colour + 
                            (get_indirect_light(render_config, &closest_hit, scene_object, closest_hit_index.unwrap(), render_config.screenshot_indirect_lighting_ray_count, rng)
                            * object_surface_properties.diffusiveness);
                    }
                }
            }

            new_colour = (scene_object.get_colour(&closest_hit) * (light_colour + render_config.global_light)).proportionally_limited() * object_surface_properties.opaqueness;
        }
        if object_surface_properties.reflectiveness != 0.0 { 
            new_colour = new_colour +
            (get_colour_recursively(scene_object.get_reflection_line(&ray, &closest_hit), 
                render_config,
                is_screenshot,
                reflection_depth_remaining - 1,
                rng
            ) * object_surface_properties.reflectiveness);
        }
        if object_surface_properties.transparency != 0.0 { 
            new_colour = new_colour + 
            (get_colour_recursively(scene_object.get_transparent_line(&ray, &closest_hit), 
                render_config, 
                is_screenshot,
                reflection_depth_remaining - 1
            , rng) * object_surface_properties.transparency);
        }

        new_colour
    }
}