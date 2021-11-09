use cgmath::{num_traits::Pow, InnerSpace};

use crate::{mat::Material, ray::Ray, Vec3};

use super::HitRecord;

pub struct Sphere<'a> {
    pub centre: Vec3,
    pub radius: f64,
    pub material: Material<'a>,
}

impl Sphere<'_> {
    #[inline]
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction.0);
        let c = oc.length_squared() - self.radius.pow(2);

        let discriminant: f64 = half_b.pow(2) - a * c;
        if discriminant.is_sign_negative() {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);
        let t = root;

        let outward_normal = (p - self.centre) / self.radius;
        let (front_face, normal) = HitRecord::face_and_normal(ray, outward_normal);

        Some(HitRecord {
            p,
            normal,
            t,
            front_face,
            material: self.material.clone(),
        })
    }
}
