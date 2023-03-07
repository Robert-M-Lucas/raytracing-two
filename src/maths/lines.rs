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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct SizedLine {
    pub line: Line,
    pub scale: f64
}

#[allow(dead_code)]
impl SizedLine {
    pub fn new(point: &V3, vector: &V3, scale: f64) -> Self {
        Self {
            line: Line::new(point, vector),
            scale
        }
    }

    pub fn from_line(line: &Line, scale: f64) -> Self {
        Self {
            line: line.clone(),
            scale
        }
    }

    pub fn length(&self) -> f64 {
        self.line.vector.magnitude() * self.scale
    }
}
