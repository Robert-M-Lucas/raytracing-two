pub mod directional_light;
pub use directional_light::DirectionalLight;
pub mod point_light;
pub use point_light::PointLight;

use crate::{colour::Colour, maths::{lines::SizedLine, vectors::V3}};

pub trait Light {
    fn get_direct_ray(&self, destination: &V3) -> SizedLine;
    fn get_intensity(&self, distance: f64) -> f64;
    fn get_colour(&self) -> &Colour;
}
