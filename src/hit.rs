use crate::{Ray, Position, Material};
pub struct Hit<'a> {
    pub inRay: Ray,
    pub position: Position,
    pub normal: Ray,
    pub material: &'a dyn Material,
    pub t: f64,
}
