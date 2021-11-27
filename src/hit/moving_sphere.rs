use cgmath::InnerSpace;

use crate::{mat::Material, ray::Ray, Vec3};

use super::HitRecord;

pub struct MovingSphere {
    pub centre_start: Vec3,
    pub centre_end: Vec3,
    pub time_start: f64,
    pub time_end: f64,
    pub radius: f64,
    pub material: Material,
}

impl MovingSphere {
    #[inline]
    pub fn centre(&self, time: f64) -> Vec3 {
        self.centre_start
            + ((time - self.time_start) / (self.time_end - self.time_start))
                * (self.centre_end - self.centre_start)
    }

    #[inline]
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.centre(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction.0);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant: f64 = half_b.powi(2) - a * c;
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

        let outward_normal = (p - self.centre(ray.time)) / self.radius;
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
