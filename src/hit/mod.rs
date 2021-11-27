pub mod list;
pub mod sphere;

use std::hint::unreachable_unchecked;

use cgmath::InnerSpace;

use crate::{mat::Material, ray::Ray, Vec3};

use self::list::HittableList;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    #[inline]
    pub fn face_and_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let ff = ray.direction.dot(outward_normal.0).is_sign_negative();
        if ff {
            (ff, outward_normal)
        } else {
            (ff, -outward_normal)
        }
    }
}

pub enum Hittable {
    Sphere(sphere::Sphere),
    #[allow(dead_code)]
    List(HittableList),
}

impl Hittable {
    #[inline]
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
            // Safety:
            // This variant is never constructed
            Hittable::List(_) => unsafe { unreachable_unchecked() },
        }
    }
}
