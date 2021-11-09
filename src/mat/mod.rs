pub mod lambertian;
pub mod metal;

use crate::{hit::HitRecord, ray::Ray, Vec3};

use self::{lambertian::Lambertian, metal::Metal};

#[derive(Clone)]
pub enum Material<'a> {
    Lambertian(&'a Lambertian),
    Metal(&'a Metal),
}

impl Material<'_> {
    #[inline]
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Vec3, Ray) {
        match self {
            Material::Lambertian(l) => l.scatter(ray, rec),
            Material::Metal(m) => m.scatter(ray, rec),
        }
    }
}
