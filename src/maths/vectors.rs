use std::f64::consts::PI;
use std::ops;
use std::cmp;
use rand::Rng;
use rand::rngs::ThreadRng;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[allow(dead_code)]
impl V3 {

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn clone(&self) -> V3 { V3 { x: self.x, y: self.y, z: self.z } }
    
    pub fn normalised(&self) -> V3 {
        let diff = 1.0 / self.magnitude();
        V3::new(self.x * diff, self.y * diff, self.z * diff)
    }

    pub fn magnitude(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn dot(&self, rhs: &V3) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn cross(&self, rhs: &V3) -> V3 {
        V3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn angle_to(&self, rhs: &V3) -> f64 {
        (self.dot(rhs) / (self.magnitude() * rhs.magnitude())).acos()
    }

    pub fn cos_angle_to(&self, rhs: &V3) -> f64 {
        self.dot(rhs) / (self.magnitude() * rhs.magnitude())
    }

    pub fn rotate_y(self, about: &V3, radians: f64) -> V3 {
        let centered = self.clone() - about;

        &V3::new(
            centered.x * radians.cos() + centered.z * radians.sin(), 
            centered.y,
            -centered.x * radians.sin() + centered.z * radians.cos()
        ) 
        + about
    }

    pub fn rotate_z(self, about: &V3, radians: f64) -> V3 {
        let centered = self - about;

        &V3::new(
            centered.x * radians.cos() - centered.y * radians.sin(), 
            centered.x * radians.sin() + centered.y * radians.cos(),
            centered.z
        ) 
        + about
    }

    pub fn reflected(&self, normal: &V3) -> V3 {
        let n = normal.normalised();
        self.clone() - (n * (2.0 * (self.dot(&n))))
    }

    pub fn get_random(direction: &V3, spread: f64, rng: &mut ThreadRng) -> V3 {
        let b3 = direction.normalised();
        let different;
        if b3.x < 0.5 { different = V3::new(1.0, 0.0, 0.0); }
        else { different = V3::new(0.0, 1.0, 0.0); }
        let b1 = b3.cross(&different).normalised();
        let b2 = b1.cross(&b3).normalised();

        let z = rng.gen_range(spread.cos()..1.0);
        let r = (1.0 - (z * z)).sqrt();
        let theta = rng.gen_range(-PI..PI);
        let x = r * theta.cos();
        let y = r * theta.sin();

        (b1 * x) + (b2 * y) + (b3 * z)
    }

    #[allow(non_snake_case)]
    pub const ZERO: V3 = V3 { x: 0.0, y: 0.0, z: 0.0};
    #[allow(non_snake_case)]
    pub const ONE: V3 = V3 { x: 1.0, y: 1.0, z: 1.0};
    #[allow(non_snake_case)]
    pub const FORWARD: V3 = V3 { x: 1.0, y: 0.0, z: 0.0};
    #[allow(non_snake_case)]
    pub const BACK: V3 = V3 { x: -1.0, y: 0.0, z: 0.0};
    #[allow(non_snake_case)]
    pub const UP: V3 = V3 { x: 0.0, y: 1.0, z: 0.0};
    #[allow(non_snake_case)]
    pub const DOWN: V3 = V3 { x: 0.0, y: -1.0, z: 0.0};
    #[allow(non_snake_case)]
    pub const LEFT: V3 = V3 { x: 0.0, y: 0.0, z: -1.0};
    #[allow(non_snake_case)]
    pub const RIGHT: V3 = V3 { x: 0.0, y: 0.0, z: 1.0};
}

impl ops::Add<V3> for V3 {
    type Output = V3;

    fn add(self, rhs: V3) -> V3 {
        V3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Add<&V3> for V3 {
    type Output = V3;

    fn add(self, rhs: &V3) -> V3 {
        V3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Add<V3> for &V3 {
    type Output = V3;

    fn add(self, rhs: V3) -> V3 {
        V3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Add<&V3> for &V3 {
    type Output = V3;

    fn add(self, rhs: &V3) -> V3 {
        V3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}


impl ops::Sub<V3> for V3 {
    type Output = V3;

    fn sub(self, rhs: V3) -> V3 {
        V3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<&V3> for V3 {
    type Output = V3;

    fn sub(self, rhs: &V3) -> V3 {
        V3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<V3> for &V3 {
    type Output = V3;

    fn sub(self, rhs: V3) -> V3 {
        V3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<&V3> for &V3 {
    type Output = V3;

    fn sub(self, rhs: &V3) -> V3 {
        V3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<V3> for V3 {
    type Output = V3;

    fn mul(self, rhs: V3) -> V3 {
        V3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<&V3> for V3 {
    type Output = V3;

    fn mul(self, rhs: &V3) -> V3 {
        V3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<V3> for &V3 {
    type Output = V3;

    fn mul(self, rhs: V3) -> V3 {
        V3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<&V3> for &V3 {
    type Output = V3;

    fn mul(self, rhs: &V3) -> V3 {
        V3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<f64> for V3 {
    type Output = V3;

    fn mul(self, rhs: f64) -> V3 {
        V3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<f64> for &V3 {
    type Output = V3;

    fn mul(self, rhs: f64) -> V3 {
        V3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Div<V3> for V3 {
    type Output = V3;

    fn div(self, rhs: V3) -> V3 {
        V3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl ops::Div<&V3> for V3 {
    type Output = V3;

    fn div(self, rhs: &V3) -> V3 {
        V3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl ops::Div<V3> for &V3 {
    type Output = V3;

    fn div(self, rhs: V3) -> V3 {
        V3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl ops::Div<&V3> for &V3 {
    type Output = V3;

    fn div(self, rhs: &V3) -> V3 {
        V3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl ops::Div<f64> for V3 {
    type Output = V3;

    fn div(self, rhs: f64) -> V3 {
        V3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Div<f64> for &V3 {
    type Output = V3;

    fn div(self, rhs: f64) -> V3 {
        V3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl cmp::PartialEq<V3> for V3 {
    fn eq(&self, rhs: &V3) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

impl cmp::Eq for V3 {}