use std::ops;

pub mod colour_getters;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Colour {
    r: f64,
    g: f64,
    b: f64
}

#[allow(dead_code)]
impl Colour {
    pub fn from_f64(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
        Self { r: r as f64 / 255.0, g: g as f64 / 255.0, b: b as f64 / 255.0 }
    }

    pub fn as_u8(&self) -> (u8, u8, u8) {
        ((self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8)
    }

    pub const BLACK: Colour = Colour { r: 0.0, g: 0.0, b: 0.0 };
}

impl ops::Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Colour {
        Colour::from_f64(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl ops::Mul<&Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: &Colour) -> Colour {
        Colour::from_f64(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl ops::Mul<Colour> for &Colour {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Colour {
        Colour::from_f64(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl ops::Mul<&Colour> for &Colour {
    type Output = Colour;

    fn mul(self, rhs: &Colour) -> Colour {
        Colour::from_f64(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl ops::Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Colour {
        Colour::from_f64(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl ops::Mul<f64> for &Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Colour {
        Colour::from_f64(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl ops::Add<Colour> for Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Colour {
        Colour::from_f64(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl ops::Add<&Colour> for Colour {
    type Output = Colour;

    fn add(self, rhs: &Colour) -> Colour {
        Colour::from_f64(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl ops::Add<Colour> for &Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Colour {
        Colour::from_f64(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl ops::Add<&Colour> for &Colour {
    type Output = Colour;

    fn add(self, rhs: &Colour) -> Colour {
        Colour::from_f64(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}