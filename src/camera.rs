use crate::vec3::*;
use crate::ray::Ray;
use crate::{Hittable, Material};

extern crate rand;
use rand::{Rng, thread_rng};

pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Colour>>,
    pub viewportWidth: f64,
    pub viewportHeight: f64,
    pub lowerLeftCorner: Position,
    pub origin: Position,
}

impl Camera {
    pub fn write_colour(&mut self, w: u32, h: u32, samplesPerPixel: u32, WORLD: &[Box<dyn Hittable>]) {
        let mut rng = thread_rng();
        let mut colourSum = Colour::blank();
        for _ in 0..samplesPerPixel {
            let origin = self.origin;
            let direction = self.lowerLeftCorner.to_direction() + Direction::new(self.viewportWidth, 0.0, 0.0)*(w as f64/self.width as f64) - Direction::new(0.0, self.viewportHeight, 0.0)*(h as f64/self.height as f64) + Direction::new(0.0, 0.0, -1.0);
            let direction = direction + Direction::new(rng.gen_range(0.0, self.viewportWidth as f64/self.width as f64),  rng.gen_range(0.0, self.viewportHeight as f64/self.height as f64), -1.0);
            let ray = Ray::new_normalised( origin, direction);
            colourSum = colourSum + ray.colour(WORLD, 50);
        }
        self.pixels[h as usize][w as usize] = colourSum / samplesPerPixel as f64;

    }
}
