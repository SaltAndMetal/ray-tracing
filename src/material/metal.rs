use crate::{Colour, Ray, Hit, Material, Vec3};

pub struct Metal {
    colour: Colour,
    fuzziness: f64,
}

impl Metal {
    pub const fn new(colour: Colour, fuzziness: f64) -> Metal {
        Metal{colour, fuzziness}
    }
}

impl Material for Metal {
    fn bounce(&self, hit: &Hit) -> Ray {
        let direction = hit.inRay.direction - hit.normal.direction*2.0*hit.inRay.direction.dot(&hit.normal.direction);
        let direction = direction + Vec3::random_in_unit_sphere()*self.fuzziness;
        Ray{ origin: hit.position, direction }
    }
    fn colour(&self) -> Colour {
        self.colour
    }
}
