pub mod dielectric;
pub mod lambertian;
pub mod metal;

use std::sync::Arc;

use crate::{hit::HitRecord, ray::Ray, Vec3};

use self::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

#[derive(Clone)]
pub enum Material {
    Lambertian(Arc<Lambertian>),
    Metal(Arc<Metal>),
    Dielectric(Dielectric),
}

impl Material {
    #[inline]
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Vec3, Ray) {
        match self {
            Material::Lambertian(l) => l.scatter(ray, rec),
            Material::Metal(m) => m.scatter(ray, rec),
            Material::Dielectric(d) => d.scatter(ray, rec),
        }
    }
}
