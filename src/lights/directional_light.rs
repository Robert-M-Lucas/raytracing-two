use crate::{maths::{vectors::V3, lines::SizedLine}, colour::Colour};

use super::Light;

const INFINITY: f64 = 1_000_000.0;

pub struct DirectionalLight {
    direction: V3,
    colour: Colour,
    intensity: f64
}

impl DirectionalLight {
    pub fn new(direction: &V3, colour: &Colour, intensity: f64) -> Self {
        Self { direction: direction.clone(), colour: colour.clone(), intensity }
    }
}

impl Light for DirectionalLight {
    fn get_colour(&self) -> &Colour {
        &self.colour
    }

    fn get_direct_ray(&self, destination: &V3) -> SizedLine {
        SizedLine::new(&(destination - (self.direction * INFINITY)), &self.direction, INFINITY)
    }

    fn get_intensity(&self, _distance: f64) -> f64 {
        self.intensity
    }
}