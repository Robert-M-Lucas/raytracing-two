
pub trait Light {
    fn get_direct_ray(&self, destination: &V3) -> SizedLine;
    fn get_intensity(&self, distance: f64) -> f64;
    fn get_colour(&self) -> &Colour;
}