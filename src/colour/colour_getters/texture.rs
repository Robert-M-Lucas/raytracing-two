use std::ops::{Index, IndexMut};
use image;

use super::ColourGetter;
use super::super::Colour;

#[allow(dead_code)]
pub struct Texture {
    size: (usize, usize),
    data: Vec<Colour>
}

#[allow(dead_code)]
impl Texture {
    pub fn new(image_path: &str) -> Result<Self, String> {
        println!("Loading texture [{}]", image_path);

        let img = image::open(image_path);

        if img.is_err() {
            println!("Loading texture failed [{}]", image_path);
            panic!("{}", format!("Loading texture failed [{}]", image_path))
        }

        let img = img.unwrap().to_rgb8();

        let dimensions = (img.dimensions().0 as usize, img.dimensions().1 as usize);
        let size = dimensions.0 * dimensions.1;
        let raw_data = img.into_raw();
        let mut data = Vec::with_capacity(size);

        for i in 0..size {
            data.push(Colour::from_u8(raw_data[i*3], raw_data[i*3+1], raw_data[i*3+2]));
        }

        Ok(Self {
            size: dimensions,
            data,
        })
    }
}

impl ColourGetter for Texture {
    fn get_colour(&self, position: (f64, f64)) -> &Colour {
        let position: (usize, usize) = ((position.0.abs().fract() * (self.size.0 as f64)).floor() as usize, 
            (position.1.abs().fract() * (self.size.1 as f64)).floor() as usize);
        &self.data[position.1 * self.size.1 + position.0]
    }
}

impl Index<(f64, f64)> for Texture {
    type Output = Colour;
    fn index<'a>(&'a self, position: (f64, f64)) -> &'a Colour {
        &self.get_colour(position)
    }
}

impl Index<(usize, usize)> for Texture {
    type Output = Colour;
    fn index<'a>(&'a self, i: (usize, usize)) -> &'a Colour {
        &self.data[i.1 * self.size.1 + i.0]
    }
}

impl IndexMut<(usize, usize)> for Texture {
    fn index_mut<'a>(&'a mut self, i: (usize, usize)) -> &'a mut Colour {
        &mut self.data[i.1 * self.size.1 + i.0]
    }
}