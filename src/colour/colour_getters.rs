pub mod solid_colour;
pub mod texture;
pub use solid_colour::SolidColour;
pub use  texture::Texture;

use super::Colour;


pub trait ColourGetter {
    fn get_colour(&self, position: (f64, f64)) -> &Colour;
}