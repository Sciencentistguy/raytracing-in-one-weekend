pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::{hit::HitRecord, ray::Ray, Vec3};

use self::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

#[derive(Clone)]
pub enum Material<'a> {
    Lambertian(&'a Lambertian),
    Metal(&'a Metal),
    Dielectric(Dielectric),
}

impl Material<'_> {
    #[inline]
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Vec3, Ray) {
        match self {
            Material::Lambertian(l) => l.scatter(ray, rec),
            Material::Metal(m) => m.scatter(ray, rec),
            Material::Dielectric(d) => d.scatter(ray, rec),
        }
    }
}
