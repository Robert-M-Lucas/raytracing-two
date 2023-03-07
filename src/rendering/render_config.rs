use crate::{objects::Object, colour::{colour_getters::ColourGetter, Colour}, lights::Light};


pub struct RenderConfig {
    pub resolution: (u32, u32),
    pub screenshot_resolution: (u32, u32),
    pub max_reflections: u32,
    pub screenshot_max_reflection: u32,
    pub sky_height: f64,
    pub sky_scale: f64,
    pub sky_texture: Box<dyn ColourGetter>,
    pub global_light: Colour,
    pub scene_objects: Vec<Box<dyn Object>>,
    pub scene_lights: Vec<Box<dyn Light>>,
    pub enable_full_bright: bool,
    pub screenshot_enable_full_bright: bool,
    pub enable_direct_lighting: bool,
    pub screenshot_enable_direct_lighting: bool,
    pub enable_indirect_lighting: bool,
    pub screenshot_enable_indirect_lighting: bool,
}