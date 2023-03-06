use colour::{Colour, colour_getters::Texture};
use interface::Interface;
use maths::vectors::V3;
use objects::{Sphere, SurfaceType, Object, Plane};
use rendering::{RenderConfig, Camera};

mod maths;
mod colour;
mod objects;
mod rendering;
mod interface;

const _8K: (u32, u32) = (7680, 4320);
const _4K: (u32, u32) = (3840, 2160);
const _1440P: (u32, u32) = (2560, 1440);
const _1080P: (u32, u32) = (1920, 1080);
const _720P: (u32, u32) = (1280, 720);
const _480P: (u32, u32) = (640, 480);
const _360P: (u32, u32) = (480, 360);
const _240P: (u32, u32) = (320, 240);
const _144P: (u32, u32) = (192, 144);

fn main() {
    let scene_objects: Vec<Box<dyn Object>> = vec![
        Box::new(Plane::new(
            &V3::new(0.0, 0.0, 0.0), 
            &(&V3::FORWARD * 5.0), 
            &(&V3::RIGHT * 5.0), 
            None, 
            Box::new(Texture::new("static\\textures\\prototype2.png").unwrap()),
            SurfaceType::new(0.0, 0.0, 1.0),
        )),
        Box::new(Sphere::new(V3::UP, 1.0, Colour::from_f64(0.0, 0.0, 1.0), SurfaceType::new(0.9, 0.1, 1.52)))
    ];

    let render_config: RenderConfig = RenderConfig { 
        resolution: _480P, 
        screenshot_resolution: _4K, 
        max_reflections: 2, 
        screenshot_max_reflection: 10, 
        sky_height: 1000.0, 
        sky_scale: 5000.0, 
        sky_texture: Box::new(Texture::new("static\\textures\\sky_prototype.png").unwrap()), 
        scene_objects: scene_objects
    };

    let camera = Camera::new(&V3::new(-5.0, 1.0, 0.0), (0.0, 0.0), 0.866);

    Interface::start(render_config, camera);
}
