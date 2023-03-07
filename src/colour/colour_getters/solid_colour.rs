use super::{super::Colour, ColourGetter};

pub struct SolidColour {
    pub colour: Colour
}

impl ColourGetter for SolidColour {
    fn get_colour(&self, _: (f64, f64)) -> &Colour {
        &self.colour
    }
}