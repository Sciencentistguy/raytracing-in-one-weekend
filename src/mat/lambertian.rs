use crate::{colour::Colour, hit::HitRecord, ray::Ray, Vec3};

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Colour,
}

impl Lambertian {
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Colour, Ray) {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray {
            origin: rec.p,
            direction: scatter_direction,
        };
        (self.albedo, scattered)
    }
}
