use crate::colour::Colour;
use crate::maths::{vectors::V3, Line};
use crate::maths::Intersection;
use super::SurfaceType;
use super::Object;


#[allow(dead_code)]
pub struct Sphere {
    pub centre: V3,
    pub radius: f64,
    pub colour: Colour,
    pub surface_type: SurfaceType,
    pub cached_closest_intersect: Option<Vec<Intersection>>,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(centre: V3, radius: f64, colour: Colour, surface_type: SurfaceType) -> Self {
        Self {
            centre: centre,
            radius: radius,
            colour: colour,
            surface_type: surface_type,
            cached_closest_intersect: None
        }
    }
}

#[allow(dead_code)]
impl Object for Sphere {
    // fn clone_object(&self) -> &(dyn SceneObject + Sync) {
    //     &Sphere::new(self.centre, self.radius, self.colour, self.light_behaviour)
    // }

    fn as_any(&self) -> &dyn Object { self }

    fn get_surface_type(&self) -> &SurfaceType {
        &self.surface_type
    }

    fn get_intersections(&self, line: &Line) -> Vec<Intersection> {
        let a = line.vector.dot(&line.vector);
        if a == 0.0 { return Vec::new(); }

        let b = 2.0 * (line.vector.dot(&line.point) - line.vector.dot(&self.centre));
        let c = line.point.dot(&line.point) + (-2.0 * line.point.dot(&self.centre)) + self.centre.dot(&self.centre) + (-(self.radius * self.radius));

        let under_root = b * b - 4.0 * a * c;

        if under_root < 0.0 { return Vec::new(); }

        let sol_1 = (-b + under_root.sqrt()) / (2.0 * a);
        let int_1 = Intersection::new(line, sol_1, &line.scale(sol_1));
        if  under_root == 0.0 { return vec!(int_1.clone(), int_1); }

        let sol_2 = (-b - under_root.sqrt()) / (2.0 * a);
        let int_2 = Intersection::new(line, sol_2, &line.scale(sol_2));

        vec!(int_1, int_2)
    }

/*
    fn get_intersections_and_cache(&mut self, line: &Line) -> Vec<Intersection> {
        let a = line.vector.dot(&line.vector);
        if a == 0.0 {
            self.cached_closest_intersect = None;
            return Vec::new();
        }

        let b = 2.0 * (line.vector.dot(&line.point) - line.vector.dot(&self.centre));
        let c = &line.point.dot(&line.point) + (-2.0 * line.point.dot(&self.centre)) + self.centre.dot(&self.centre) + (-(self.radius * self.radius));

        let under_root = b * b - 4.0 * a * c;

        if under_root < 0.0 { 
            self.cached_closest_intersect = None;
            return Vec::new(); 
        }

        let sol_1 = (-b + under_root.sqrt()) / (2.0 * a);
        let int_1 = Intersection::new(sol_1, &line.scale(sol_1), &self.centre);
        if under_root == 0.0 { 
            self.cached_closest_intersect = Some(vec!(int_1.clone()));
            return vec!(int_1); 
        }

        let sol_2 = (-b - under_root.sqrt()) / (2.0 * a);
        let int_2 = Intersection::new(sol_2, &line.scale(sol_2), &self.centre);

        let out = vec!(int_1, int_2);
        let closest = Intersection::closest(&out, 0.0);
        if closest.is_none() { self.cached_closest_intersect = None }
        else {
            self.cached_closest_intersect = Some(vec!(out[0].clone(), out[1].clone()));
        }
        out
    }
*/

    fn get_colour(&self, _intersection: &Intersection) -> &Colour {
        &self.colour
    }

    fn get_reflection_line(&self, line: &Line, intersection: &Intersection) -> Line {
        Line::new(&intersection.position, 
            &line.vector.reflected(&(self.centre.clone() - intersection.position)))
    }

    fn get_transparent_line(&self, line: &Line, intersection: &Intersection) -> Line {
        let theta1 = line.vector.angle_to(&(self.centre.clone() - &intersection.position));
        let theta2 = (theta1.sin() / self.surface_type.refractive_index).asin();
        let a = self.radius * theta2.cos();
        let b = a / (theta1 - theta2).cos();

        let reflection_point = &intersection.position + (line.vector.normalised() * b);

        let new_vector = line.vector.reflected(&(reflection_point.clone() - &self.centre.clone()));
        let new_point = &reflection_point + &(&new_vector.normalised() * (b));

        Line::new(&new_point, &new_vector.normalised())
    }

    /*
    fn get_colours(&self, incoming_ray: &Line) -> Option<((u8, u8, u8), f64, Option<Line>)> {
        let intersections = self.get_intersections(incoming_ray);
        let hit = Intersection::closest(&intersections, 0.0);
        if hit.is_none() { return None; }

        let hit = hit.unwrap();
        let outgoing_line;
        let alpha;

        match self.light_behaviour {
            LightBehaviour::Solid => { outgoing_line = None; alpha = 1.0; },
            LightBehaviour::Reflective(a) => { 
                outgoing_line = Some(Line::new(&hit.position, &incoming_ray.vector.reflected(&(self.centre.clone() - &hit.position))));
                alpha = a;
            },
            LightBehaviour::Transparent(alph, r) => {
                let theta1 = incoming_ray.vector.angle_to(&(self.centre.clone() - &hit.position));
                let theta2 = (theta1.sin() / r).asin();
                let a = self.radius * theta2.cos();
                let b = a / (theta1 - theta2).cos();

                let reflection_point = &hit.position + &(&incoming_ray.vector.normalised() * b);

                let new_vector = incoming_ray.vector.reflected(&(reflection_point.clone() - &self.centre.clone()));
                let new_point = &reflection_point + &(&new_vector.normalised() * (b));

                outgoing_line = Some(Line::new(&new_point, &new_vector.normalised()));

                alpha = alph;
            }
        }

        Some((self.colour, alpha, outgoing_line))
    }

    fn get_colour_using_cache(&self, incoming_ray: &Line) -> Option<((u8, u8, u8), f64, Option<Line>)> {
        let hit = Intersection::closest(self.cached_closest_intersect.as_ref().unwrap(), 0.0).clone().unwrap();
        let outgoing_line;
        let alpha;

        match self.light_behaviour {
            LightBehaviour::Solid => { outgoing_line = None; alpha = 1.0; },
            LightBehaviour::Reflective(a) => { 
                outgoing_line = Some(Line::new(&hit.position, &incoming_ray.vector.reflected(&(self.centre.clone() - &hit.position))));
                alpha = a;
            },
            LightBehaviour::Transparent(alph, r) => {
                let theta1 = incoming_ray.vector.angle_to(&(self.centre.clone() - &hit.position));
                let theta2 = (theta1.sin() / r).asin();
                let a = self.radius * theta2.cos();
                let b = a / (theta1 - theta2).cos();

                let reflection_point = &hit.position + &(&incoming_ray.vector.normalised() * b);

                let new_vector = incoming_ray.vector.reflected(&(reflection_point.clone() - &self.centre.clone()));
                let new_point = &reflection_point + &(&new_vector.normalised() * (b));

                outgoing_line = Some(Line::new(&new_point, &new_vector.normalised()));

                alpha = alph;
            }
        }

        Some((self.colour, alpha, outgoing_line))
    }
    */
}