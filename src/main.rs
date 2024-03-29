use colour::{Colour, colour_getters::{Texture, SolidColour}};
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

const _16K: (u32, u32) = (15360, 8640);
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
    let scene_objects: Vec<Box<dyn Object + Sync>> = vec![
        // Floor
        Box::new(Plane::new(
            &V3::new(0.0, 0.0, 0.0), 
            &(&V3::FORWARD * 5.0), 
            &(&V3::RIGHT * 5.0), 
            None, 
            Box::new(Texture::new("static/textures/prototype2.png").unwrap()),
            SurfaceType::new(0.0, 0.0, 0.0, 1.0, true, false),
        )),
        
        // Mirror
        Box::new(Plane::new(
            &V3::new(7.0, 8.0, 0.0), &V3::new(1.0, 0.0, 1.0).normalised(), &V3::new(-1.0, 6.0, 1.0).normalised(), 
            Some((-6.0, -4.0, 6.0, 4.0)), 
            Box::new(SolidColour { colour: Colour::BLACK }),
            SurfaceType::new(0.0, 0.7, 0.0, 1.54, true, false)
        )),

        // Spheres
        Box::new(Sphere::new(V3::new(2.0 , 1.0, 0.0), 0.5, Colour::RED, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(2.0 , 1.0, 4.0), 0.5, Colour::RED, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(2.0 , 1.0, 2.0), 0.5, Colour::BLUE, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(-2.0, 1.0, 4.0), 0.5, Colour::BLUE, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(0.0 , 1.0, 2.0), 0.5, Colour::GREEN, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(-2.0, 1.0, 0.0), 0.5, Colour::GREEN, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(0.0 , 1.0, 0.0), 0.5, Colour::from_u8(44, 90, 100), SurfaceType::new(0.0, 0.0, 0.7, 1.52, false, false))),
        Box::new(Sphere::new(V3::new(0.0 , 1.0, 4.0), 0.5, Colour::from_u8(44, 90, 100), SurfaceType::new(0.0, 0.0, 0.7, 1.52, false, false))),
        Box::new(Sphere::new(V3::new(-2.0, 1.0, 2.0), 0.5, Colour::BLACK, SurfaceType::new(0.0, 0.8, 0.0, 1.52, false, false))),

        Box::new(Sphere::new(V3::new(2.0 , 1.0, 0.0 + 6.0), 0.5, Colour::RED, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(2.0 , 1.0, 4.0 + 6.0), 0.5, Colour::RED, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(2.0 , 1.0, 2.0 + 6.0), 0.5, Colour::BLUE, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(-2.0, 1.0, 4.0 + 6.0), 0.5, Colour::BLUE, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(0.0 , 1.0, 2.0 + 6.0), 0.5, Colour::WHITE, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(-2.0, 1.0, 0.0 + 6.0), 0.5, Colour::GREEN, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(0.0 , 1.0, 0.0 + 6.0), 0.5, Colour::from_u8(44, 90, 100), SurfaceType::new(0.0, 0.0, 0.7, 1.52, false, false))),
        Box::new(Sphere::new(V3::new(0.0 , 1.0, 4.0 + 6.0), 0.5, Colour::from_u8(44, 90, 100), SurfaceType::new(0.0, 0.0, 0.7, 1.52, false, false))),
        Box::new(Sphere::new(V3::new(-2.0, 1.0, 2.0 + 6.0), 0.5, Colour::BLACK, SurfaceType::new(0.0, 0.8, 0.0, 1.52, false, false))),

        Box::new(Sphere::new(V3::new(2.0  + 6.0, 1.0, 0.0), 0.5, Colour::RED, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(2.0  + 6.0, 1.0, 4.0), 0.5, Colour::RED, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(2.0  + 6.0, 1.0, 2.0), 0.5, Colour::WHITE, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(-2.0 + 6.0, 1.0, 4.0), 0.5, Colour::BLUE, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(0.0  + 6.0, 1.0, 2.0), 0.5, Colour::GREEN, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(-2.0 + 6.0, 1.0, 0.0), 0.5, Colour::GREEN, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(0.0  + 6.0, 1.0, 0.0), 0.5, Colour::from_u8(44, 90, 100), SurfaceType::new(0.0, 0.0, 0.7, 1.52, false, false))),
        Box::new(Sphere::new(V3::new(0.0  + 6.0, 1.0, 4.0), 0.5, Colour::from_u8(44, 90, 100), SurfaceType::new(0.0, 0.0, 0.7, 1.52, false, false))),
        Box::new(Sphere::new(V3::new(-2.0 + 6.0, 1.0, 2.0), 0.5, Colour::BLACK, SurfaceType::new(0.0, 0.8, 0.0, 1.52, false, false))),

        Box::new(Sphere::new(V3::new(2.0  + 6.0, 1.0, 0.0 + 6.0), 0.5, Colour::WHITE, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(2.0  + 6.0, 1.0, 4.0 + 6.0), 0.5, Colour::RED, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(2.0  + 6.0, 1.0, 2.0 + 6.0), 0.5, Colour::BLUE, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(-2.0 + 6.0, 1.0, 4.0 + 6.0), 0.5, Colour::BLUE, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(0.0  + 6.0, 1.0, 2.0 + 6.0), 0.5, Colour::GREEN, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(-2.0 + 6.0, 1.0, 0.0 + 6.0), 0.5, Colour::GREEN, SurfaceType::new(1.0, 0.0, 0.0, 1.0, true, false))),
        Box::new(Sphere::new(V3::new(0.0  + 6.0, 1.0, 0.0 + 6.0), 0.5, Colour::from_u8(44, 90, 100), SurfaceType::new(0.0, 0.0, 0.7, 1.52, false, false))),
        Box::new(Sphere::new(V3::new(0.0  + 6.0, 1.0, 4.0 + 6.0), 0.5, Colour::from_u8(44, 90, 100), SurfaceType::new(0.0, 0.0, 0.7, 1.52, false, false))),
        Box::new(Sphere::new(V3::new(-2.0 + 6.0, 1.0, 2.0 + 6.0), 0.5, Colour::BLACK, SurfaceType::new(0.0, 0.8, 0.0, 1.52, false, false))),

        // Big Sphere
        Box::new(Sphere::new(V3::new(3.0, 3.0, 5.0), 2.0, Colour::from_u8(44, 90, 100), SurfaceType::new(0.0, 0.0, 0.7, 1.52, false, false))),

        // Light Sphere
        Box::new(Sphere::new(V3::new(1.5, 2.0, -1.5), 0.2, Colour::from_f64(1.0, 1.0, 0.0), SurfaceType::new(0.0, 0.0, 0.0, 1.52, false, true))),

        // Walls
        Box::new(Plane::new(&V3::new(10.0, 2.0, 4.0), &V3::new(0.0, 1.0, 0.0), &V3::new(0.0, 0.0, 1.0), Some((0.0, 0.0, 4.0, 4.0)), 
            Box::new(SolidColour { colour: Colour::RED }),
            SurfaceType::new(1.0, 0.0, 0.0, 1.54, true, false)
        )),
        Box::new(Plane::new(&V3::new(6.0, 6.0, 8.0), &V3::new(0.0, 1.0, 0.0), &V3::new(-1.0, 0.0, 0.0), Some((0.0, 0.0, 4.0, 4.0)), 
            Box::new(SolidColour { colour: Colour::WHITE }),
            SurfaceType::new(1.0, 0.0, 0.0, 1.54, true, false)
        )),
    ];

    let scene_lights: Vec<Box<dyn Light + Sync>> = vec![
        Box::new(DirectionalLight::new(&V3::new(1.0, -1.0, 1.0), &Colour::from_u8(255, 235, 200), 1.5)),
        Box::new(DirectionalLight::new(&V3::new(0.0, -1.0, 0.0), &Colour::from_u8(205, 247, 247), 0.4)),
        Box::new(PointLight::new(&V3::new(1.5, 2.0, -1.5), &Colour::from_f64(1.0, 1.0, 0.0), 100.0))
    ];

    let render_config: RenderConfig = RenderConfig { 
        resolution: _240P, 
        screenshot_resolution: _1080P,
        max_reflections: 2, 
        screenshot_max_reflection: 10, 
        // diffusive_constant: 1.0 / (4.0 * PI * 10.0),
        sky_height: 1000.0, 
        sky_scale: 5000.0, 
        sky_texture: Box::new(Texture::new("static/textures/sky_prototype.png").unwrap()),
        global_light: Colour::BLACK, // Colour::from_f64(0.2, 0.2, 0.2),
        scene_objects,
        scene_lights,
        enable_full_bright: false,
        screenshot_enable_full_bright: false,
        enable_direct_lighting: true,
        screenshot_enable_direct_lighting: true,
        indirect_lighting_ray_count: 0, 
        screenshot_indirect_lighting_ray_count: 500,
    };

    let camera = Camera::new(&V3::new(-5.0, 1.0, 0.0), (0.0, 0.0), 0.866);

    Interface::start(render_config, camera);
}
