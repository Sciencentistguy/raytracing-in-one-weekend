pub mod list;
pub mod sphere;

use std::hint::unreachable_unchecked;

use cgmath::InnerSpace;

use crate::{mat::Material, ray::Ray, Vec3};

use self::list::HittableList;

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material<'a>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord<'_> {
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

pub enum Hittable<'a> {
    Sphere(sphere::Sphere<'a>),
    List(HittableList<'a>),
}

impl Hittable<'_> {
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
