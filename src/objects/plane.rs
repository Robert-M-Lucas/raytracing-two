use crate::colour::Colour;
use crate::maths::lines::Line;
use crate::maths::Intersection;
use crate::maths::vectors::V3;
use crate::colour::colour_getters::ColourGetter;
use super::Object;
use super::SurfaceType;

#[allow(dead_code)]
pub struct Plane {
    pub point: V3,
    pub vector_one: V3,
    pub vector_two: V3,
    pub limits: Option<(f64, f64, f64, f64)>, // x-min, y-min, x-max, y-max
    colour_getter: Box<dyn ColourGetter + Sync>,
    surface_type: SurfaceType,
    cached_sol: f64
}

#[allow(dead_code)]
impl Plane {
    pub fn new(point: &V3, vector_one: &V3, vector_two: &V3, limits: Option<(f64, f64, f64, f64)>, colour_getter: Box<dyn ColourGetter + Sync>, surface_type: SurfaceType) -> Self {
        Self {
            point: point.clone(),
            vector_one: vector_one.clone(),
            vector_two: vector_two.clone(),
            limits,
            colour_getter,
            surface_type,
            cached_sol: f64::NAN
        }
    }

    fn get_intersections(&self, line: &Line) -> Option<(f64, f64, f64)> {
        let d1 = self.vector_one.y * self.vector_two.z - self.vector_one.z * self.vector_two.y;
        let d2 = self.vector_one.x * self.vector_two.z - self.vector_one.z * self.vector_two.x;
        let d3 = self.vector_one.x * self.vector_two.y - self.vector_one.y * self.vector_two.x;
        let d = (line.vector.x * d1) - (line.vector.y * d2) + (line.vector.z * d3);
        if d == 0.0 { return None; }

        let sol = (((self.point.x - line.point.x) * d1) -((self.point.y - line.point.y) * d2) + ((self.point.z - line.point.z) * d3)) / d;

        let mut a1 = -self.vector_one.x;
        let mut b1 = -self.vector_two.x;
        let mut a2 = -self.vector_one.y;
        let mut b2 = -self.vector_two.y;

        let c1;
        let c2;
        if a1 == 0.0 && b1 == 0.0 {
            a1 = -self.vector_one.z;
            b1 = -self.vector_two.z;

            c1 = -line.point.z + self.point.z - (sol * line.vector.z);
            c2 = -line.point.y + self.point.y - (sol * line.vector.y);
        }
        else if a2 == 0.0 && b2 == 0.0 {
            a2 = -self.vector_one.z;
            b2 = -self.vector_two.z;

            c1 = -line.point.x + self.point.x - (sol * line.vector.x);
            c2 = -line.point.z + self.point.z - (sol * line.vector.z);
        }
        else {
            c1 = -line.point.x + self.point.x - (sol * line.vector.x);
            c2 = -line.point.y + self.point.y - (sol * line.vector.y);
        }

        let det2 = a1*b2 - a2*b1;

        let sol1 = (b2*c1 - a2*c2) / det2;
        let sol2 = (-b1*c1 + a1*c2) / det2;
        
        Some((sol, sol1, sol2))
    }

    fn get_intersections_using_cache(&self, line: &Line) -> (f64, f64, f64) {
        let sol = self.cached_sol;

        let mut a1 = -self.vector_one.x;
        let mut b1 = -self.vector_two.x;
        let mut a2 = -self.vector_one.y;
        let mut b2 = -self.vector_two.y; 

        let c1;
        let c2;
        if a1 == 0.0 && b1 == 0.0 {
            a1 = -self.vector_one.z;
            b1 = -self.vector_two.z;

            c1 = -line.point.z + self.point.z - (sol * line.vector.z);
            c2 = -line.point.y + self.point.y - (sol * line.vector.y);
        }
        else if a2 == 0.0 && b2 == 0.0 {
            a2 = -self.vector_one.z;
            b2 = -self.vector_two.z;

            c1 = -line.point.x + self.point.x - (sol * line.vector.x);
            c2 = -line.point.z + self.point.z - (sol * line.vector.z);
        }
        else {
            c1 = -line.point.x + self.point.x - (sol * line.vector.x);
            c2 = -line.point.y + self.point.y - (sol * line.vector.y);
        }

        let det2 = a1*b2 - a2*b1;

        let sol1 = (b2*c1 - a2*c2) / det2;
        let sol2 = (-b1*c1 + a1*c2) / det2;
        
        (sol, sol1, sol2)
    }
}

#[allow(dead_code)]
impl Object for Plane {
    fn as_any(&self) -> &dyn Object { self }

    fn get_surface_type(&self) -> &SurfaceType {
        &self.surface_type
    }

    fn get_intersections(&self, line: &Line) -> Vec<Intersection> {
        let sols = self.get_intersections(&line);

        if sols.is_none() {
            return Vec::new();
        }

        let sols = sols.unwrap();

        if self.limits.is_some() {
            let limits = self.limits.unwrap();

            if sols.1 < limits.0 || sols.2 < limits.1 || sols.1 > limits.2 || sols.2 > limits.3 {
                return Vec::new();
            }
        }

        vec!(Intersection::new(line, sols.0, &line.scale(sols.0)))
    }

    fn get_normal(&self, intersection: &Intersection) -> V3 {
        // TODO: Do this without trial and error
        let normal = self.vector_one.cross(&self.vector_two);
        if (intersection.sized_line.line.point - (self.point + normal)).magnitude() < (intersection.sized_line.line.point - (self.point - normal)).magnitude() {
            return normal;
        }
        normal * -1.0
    }

    /*
    fn get_intersections_and_cache(&mut self, line: &Line) -> Vec<Intersection> {
        let d1 = self.vector_one.y * self.vector_two.z - self.vector_one.z * self.vector_two.y;
        let d2 = self.vector_one.x * self.vector_two.z - self.vector_one.z * self.vector_two.x;
        let d3 = self.vector_one.x * self.vector_two.y - self.vector_one.y * self.vector_two.x;
        let d = ((line.vector.x * d1)) - (line.vector.y * d2) + ((line.vector.z * d3));
        if d == 0.0 { return Vec::new(); }

        let sol = (((self.point.x - line.point.x) * d1) -((self.point.y - line.point.y) * d2) + ((self.point.z - line.point.z) * d3)) / d;
        self.cached_sol = sol;

        if self.limits.is_some() {
            let sols = self.get_intersections_using_cache(&line);

            let limits = self.limits.unwrap();

            if sols.1 < limits.0 || sols.2 < limits.1 || sols.1 > limits.2 || sols.2 > limits.3 {
                return Vec::new();
            }
        }

        vec!(Intersection::new(sol, &line.scale(sol), &V3::ZERO))
    }
    */    

    fn get_colour(&self, intersection: &Intersection) -> &Colour {
        // TODO: Massive inefficiency due to recalculation
        let sol = self.get_intersections(&intersection.sized_line.line).unwrap();
        self.colour_getter.get_colour((sol.1, sol.2))
    }

    fn get_reflection_line(&self, _line: &Line, intersection: &Intersection) -> Line {
        Line::new(&intersection.position, 
            &V3::reflected(&intersection.sized_line.line.vector, 
            &(&self.vector_one.cross(&self.vector_two)).normalised())
        )
    }

    fn get_transparent_line(&self, _line: &Line, intersection: &Intersection) -> Line {
        Line::new(&intersection.position,
            &intersection.sized_line.line.vector
        )
    }

    /*
    fn get_colour(&self, incoming_ray: &Line) -> Option<((u8, u8, u8), f64, Option<Line>)> {
        let hit = self.get_intersections(incoming_ray);
        if hit.is_none() || hit.unwrap().0 < 0.0 { return None; }
        let hit = hit.unwrap();

        let outgoing_line;
        let alpha;

        match self.surface_type {
            LightBehaviour::Solid => { outgoing_line = None; alpha = 1.0; },
            LightBehaviour::Reflective(a) => {
                outgoing_line = Some(Line::new(&incoming_ray.scale(hit.0), &V3::reflected(&incoming_ray.vector, &(&self.vector_one.cross(&self.vector_two)).normalised())));
                alpha = a;
            },
            LightBehaviour::Transparent(a, _) => {
                outgoing_line = Some(Line::new(&incoming_ray.scale(hit.0), &incoming_ray.vector));
                alpha = a;
            }
        }

        Some((*self.colour_getter.get_colour((hit.1, hit.2)), alpha, outgoing_line))
    }
    */

    /*
    fn get_colour_using_cache(&self, incoming_ray: &Line) -> Option<((u8, u8, u8), f64, Option<Line>)> {
        let hit = self.get_intersections_using_cache(incoming_ray);

        let outgoing_line;
        let alpha;

        match self.surface_type {
            LightBehaviour::Solid => { outgoing_line = None; alpha = 1.0; },
            LightBehaviour::Reflective(a) => {
                outgoing_line = Some(Line::new(&incoming_ray.scale(hit.0), &V3::reflected(&incoming_ray.vector, &(&self.vector_one.cross(&self.vector_two)).normalised())));
                alpha = a;
            },
            LightBehaviour::Transparent(a, _) => {
                outgoing_line = Some(Line::new(&incoming_ray.scale(hit.0), &incoming_ray.vector));
                alpha = a;
            }
        }

        Some((*self.colour_getter.get_colour((hit.1, hit.2)), alpha, outgoing_line))
    }

    fn get_solid_colour(&self, incoming_ray: &Line) -> (u8, u8, u8) {
        let hit = self.get_intersections_using_cache(incoming_ray);
        *self.colour_getter.get_colour((hit.1, hit.2))
    }
    */
}
