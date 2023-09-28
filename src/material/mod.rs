mod lambertian;
mod metal;
pub use lambertian::Lambertian;
pub use metal::Metal;
use crate::{Hit, Ray, Colour};

pub trait Material {
    fn bounce(&self, hit: &Hit) -> Ray;
    fn colour(&self) -> Colour;
}
