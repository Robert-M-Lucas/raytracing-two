use crate::{objects::Object, colour::colour_getters::ColourGetter};


pub struct RenderConfig {
    pub resolution: (u32, u32),
    pub screenshot_resolution: (u32, u32),
    pub max_reflections: u32,
    pub screenshot_max_reflection: u32,
    pub sky_height: f64,
    pub sky_scale: f64,
    pub sky_texture: Box<dyn ColourGetter>,
    pub scene_objects: Vec<Box<dyn Object>>,
    pub enable_direct_lighting: bool,
    pub screenshot_enable_direct_lighting: bool,
    pub enable_indirect_lighting: bool,
    pub screenshot_enable_indirect_lighting: bool,
}