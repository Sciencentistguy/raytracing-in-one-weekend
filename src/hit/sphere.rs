use cgmath::{
    num_traits::{Float, Pow},
    InnerSpace,
};

use crate::{mat::Material, point::Point, ray::Ray};

use super::HitRecord;

pub struct Sphere {
    pub centre: Point,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin.0 - self.centre.0;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction.0);
        let c = oc.length_squared() - self.radius.pow(2);

        let discriminant: f64 = half_b.pow(2) - a * c;
        if discriminant.is_sign_negative() {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let root = (-half_b - sqrtd) / a;
        let rng = t_min..t_max;
        if !rng.contains(&root) {
            let root = (-half_b + sqrtd) / a;
            if !rng.contains(&root) {
                return None;
            }
        }

        let p = ray.at(root);
        let t = root;

        let outward_normal = (p.0 - self.centre.0) / self.radius;
        let (front_face, normal) = HitRecord::face_and_normal(ray, outward_normal);

        Some(HitRecord {
            p,
            normal,
            t,
            front_face,
            material: self.material,

        })
    }
}
