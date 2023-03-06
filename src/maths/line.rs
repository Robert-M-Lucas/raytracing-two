use super::vectors::V3;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub point: V3,
    pub vector: V3
}

#[allow(dead_code)]
impl Line {
    pub fn new(point: &V3, vector: &V3) -> Self {
        Self {
            point: point.clone(),
            vector: vector.clone()
        }
    }

    pub fn scale(&self, scale: f64) -> V3 {
        return self.point + (self.vector * scale);
    }
}