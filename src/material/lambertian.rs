use crate::{Material, Vec3, Ray, Hit, Colour};
pub struct Lambertian {
    colour: Colour,
}

impl Lambertian {
    pub const fn new(colour: Colour) -> Lambertian {
        Lambertian{ colour }
    }
}

impl Material for Lambertian {
    fn bounce(&self, hit: &Hit) -> Ray {
        let direction = hit.position + hit.normal.direction + Vec3::random_in_unit_sphere();
        Ray::new_normalised(hit.position, direction)
    }
    fn colour(&self) -> Colour {
        self.colour
    }
}
