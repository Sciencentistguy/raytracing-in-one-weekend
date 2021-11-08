pub mod lambertian;
pub mod metal;

use crate::{colour::Colour, hit::HitRecord, ray::Ray};

use self::{lambertian::Lambertian, metal::Metal};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal)
}

impl Material {
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        match self {
            Material::Lambertian(l) => Some(l.scatter(ray, rec)),
            Material::Metal(m) => Some(m.scatter(ray, rec)),
        }
    }
}
