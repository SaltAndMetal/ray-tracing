use crate::vec3::*;
use crate::{Hittable, Material};

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Position,
    pub direction: Direction,
}

impl Ray {
    pub fn new_normalised(origin: Position, direction: Direction) -> Ray {
        Ray{ origin, direction: direction.normalise() }
    }
    pub fn lerp(&self, t: f64) -> Position {
        self.origin + (self.direction*t).to_position()
    }
   pub fn colour(&self, WORLD: &[Box<dyn Hittable>], depth: u32) -> Colour {
       if depth == 0 {
           return Colour::blank();
       }
       let mut returnColour = self.default_colour();
       let mut hits = Vec::new();
       for object in WORLD {
           for hit in object.intersection_points(self, Box::new(|t| t >= 0.001)) {
               hits.push(hit);
           }
       }
       hits.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
       if !hits.is_empty() {
           returnColour = hits[0].material.bounce(&hits[0]).colour(WORLD, depth-1)*hits[0].material.colour();
       }
       returnColour
   }
    pub fn default_colour(&self) -> Colour {
        let normalDirection = self.direction.normalise();
        let t = 0.5 * (normalDirection.y() + 1.0);
        Colour::new(1.0, 1.0, 1.0)*(1.0-t) + Colour::new(0.5, 0.7, 1.0)*t
    }
}

