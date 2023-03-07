pub mod plane;
pub mod sphere;
pub use plane::Plane;
pub use sphere::Sphere;

use crate::maths::Intersection;
use crate::maths::lines::Line;
use crate::colour::Colour;
use crate::maths::vectors::V3;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct SurfaceType {
    pub reflectiveness: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    pub opaqueness: f64,
    pub blocks_light: bool,
    pub full_bright: bool,
}

#[allow(dead_code)]
impl SurfaceType {
    pub fn new(reflectiveness: f64, transparency: f64, refractive_index: f64, blocks_light: bool, full_bright: bool) -> Self {
        Self { reflectiveness, transparency, refractive_index, opaqueness: 1.0 - transparency - reflectiveness, blocks_light, full_bright }
    }
}

pub trait Object {
    fn as_any(&self) -> &dyn Object;
    fn get_surface_type(&self) -> &SurfaceType;
    fn get_intersections(&self, line: &Line) -> Vec<Intersection>;
    fn get_normal(&self, intersection: &Intersection) -> V3;
    fn get_colour(&self, intersection: &Intersection) -> &Colour;
    fn get_reflection_line(&self, line: &Line, intersection: &Intersection) -> Line;
    fn get_transparent_line(&self, line: &Line, intersection: &Intersection) -> Line;
}