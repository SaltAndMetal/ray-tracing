mod sphere;
pub use sphere::Sphere;
use crate::{Ray, Hit, Material};

pub trait Hittable {
    fn intersection_points(&self, ray: &Ray, predicate: Box<dyn Fn(f64) -> bool>) -> Vec<Hit>;
    fn material(&self) -> &dyn Material;
}
