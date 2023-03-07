use colour::{Colour, colour_getters::Texture};
use interface::Interface;
use lights::{Light, DirectionalLight, PointLight};
use maths::vectors::V3;
use objects::{Sphere, SurfaceType, Object, Plane};
use rendering::{RenderConfig, Camera};

mod maths;
mod colour;
mod objects;
mod rendering;
mod interface;
mod lights;

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
            SurfaceType::new(0.0, 0.0, 1.0, true, false),
        )),
        Box::new(Sphere::new(V3::new(2.0, 1.0, 2.0), 1.0, Colour::from_f64(1.0, 1.0, 1.0), SurfaceType::new(0.0, 0.0, 1.52, true, false))),
        Box::new(Sphere::new(V3::new(-2.0, 1.0, -2.0), 1.0, Colour::from_f64(1.0, 1.0, 1.0), SurfaceType::new(0.0, 0.0, 1.52, true, false))),
        Box::new(Sphere::new(V3::new(1.5, 2.0, -1.5), 0.2, Colour::from_f64(1.0, 1.0, 0.0), SurfaceType::new(0.0, 0.0, 1.52, false, true))),
    ];

    let scene_lights: Vec<Box<dyn Light>> = vec![
        Box::new(DirectionalLight::new(&V3::new(1.0, -1.0, 1.0), &Colour::from_u8(255, 235, 200), 0.7)),
        Box::new(DirectionalLight::new(&V3::new(0.0, -1.0, 0.0), &Colour::from_u8(205, 247, 247), 0.2)),
        Box::new(PointLight::new(&V3::new(1.5, 2.0, -1.5), &Colour::from_f64(1.0, 1.0, 0.0), 10.0))
    ];

    let render_config: RenderConfig = RenderConfig { 
        resolution: _240P, 
        screenshot_resolution: _4K, 
        max_reflections: 2, 
        screenshot_max_reflection: 10, 
        sky_height: 1000.0, 
        sky_scale: 5000.0, 
        sky_texture: Box::new(Texture::new("static\\textures\\sky_prototype.png").unwrap()), 
        global_light: Colour::from_f64(0.2, 0.2, 0.2),
        scene_objects: scene_objects,
        scene_lights: scene_lights,
        enable_full_bright: false,
        screenshot_enable_full_bright: false,
        enable_direct_lighting: true,
        screenshot_enable_direct_lighting: true,
        enable_indirect_lighting: false, 
        screenshot_enable_indirect_lighting: true,
    };

    let camera = Camera::new(&V3::new(-5.0, 1.0, 0.0), (0.0, 0.0), 0.866);

    Interface::start(render_config, camera);
}
