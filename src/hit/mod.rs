pub mod list;
pub mod sphere;

use cgmath::InnerSpace;

use crate::{mat::Material, point::Point, ray::Ray, Vec3};

use self::list::HittableList;

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
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
    List(HittableList),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
            Hittable::List(list) => list.hit(ray, t_min, t_max),
        }
    }
}
