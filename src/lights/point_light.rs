use crate::{maths::{vectors::V3, lines::SizedLine}, colour::Colour};

use super::Light;

pub struct PointLight {
    position: V3,
    colour: Colour,
    intensity: f64
}

impl PointLight {
    pub fn new(position: &V3, colour: &Colour, intensity: f64) -> Self {
        Self { position: position.clone(), colour: colour.clone(), intensity }
    }
}

impl Light for PointLight {
    fn get_colour(&self) -> &Colour {
        &self.colour
    }

    fn get_direct_ray(&self, destination: &V3) -> SizedLine {
        let vector = destination - self.position;
        SizedLine::new(&self.position, &vector, 1.0)
    }

    fn get_intensity(&self, distance: f64) -> f64 {
        self.intensity / distance.powi(2)
    }
}