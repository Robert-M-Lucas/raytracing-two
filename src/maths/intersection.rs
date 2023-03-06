use super::vectors::V3;
use super::Line;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub line: Line,
    pub scale: f64,
    pub position: V3
}

#[allow(dead_code)]
impl Intersection {
    pub fn new(line: &Line, scale: f64, position: &V3) -> Self {
        Self { line: line.clone(), scale, position: position.clone() }
    }

    pub fn closest_bounded(hits: &Vec<Intersection>, near_scale: f64, far_scale: f64) -> Option<&Intersection> {
        let mut closest_dist = f64::INFINITY;
        let mut current_intersect = None;

        for i in 0..hits.len() {
            let dist = hits[i].line.vector.magnitude() * hits[i].scale;
            if dist < closest_dist && dist > near_scale && dist < far_scale {
                closest_dist = dist;
                current_intersect = Some(i);
            }
        }

        if current_intersect.is_none() { return None; }
        Some(&hits[current_intersect.unwrap()])
    }

    pub fn closest(hits: &Vec<Intersection>) -> Option<&Intersection> {
        let mut closest_dist = f64::INFINITY;
        let mut current_intersect = None;

        for i in 0..hits.len() {
            let dist = hits[i].line.vector.magnitude() * hits[i].scale;
            if dist < closest_dist && dist > 0.0 {
                closest_dist = dist;
                current_intersect = Some(i);
            }
        }

        if current_intersect.is_none() { return None; }
        Some(&hits[current_intersect.unwrap()])
    }
}