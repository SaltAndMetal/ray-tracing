use crate::Ray;
use crate::Hit;
use crate::Hittable;
use crate::Material;
use crate::Position;

pub struct Sphere<'a> {
    pub centre: Position,
    pub radius: f64,
    pub material: &'a dyn Material,
}
impl<'a> Hittable for Sphere<'a> {
    fn intersection_points(&self, ray: &Ray, predicate: Box<dyn Fn(f64) -> bool>) -> Vec<Hit> {
        let oc = (ray.origin - self.centre).to_direction();
        let a = ray.direction.len().powf(2_f64);
        let half_b = oc.dot(&ray.direction);
        let c = oc.len().powf(2_f64) - self.radius.powf(2_f64);
        let discriminant = half_b.powf(2_f64) - a*c;
        if discriminant < 0.0 {
            Vec::new()
        }
        else {
            let mut returnVec = Vec::new();
            let roots = vec![-half_b - discriminant.powf(0.5_f64)/a, -half_b + discriminant.powf(0.5_f64)/a];
            for root in roots {
                if predicate(root) {
                    let position = ray.lerp(root);
                    let direction = position - self.centre;
                    returnVec.push(Hit{ inRay: *ray, position, normal: Ray::new_normalised(position, direction), material: self.material, t: root });
                }
            }
            returnVec
        }
    }
    fn material(&self) -> &dyn Material {
        self.material
    }
}
