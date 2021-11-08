use crate::{colour::Colour, hit::HitRecord, ray::Ray, Vec3};

pub struct Metal {
    pub albedo: Colour,
    pub fuzz: f64,
}

impl Metal {
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Colour, Ray) {
        let reflected = ray.direction.unit_vec().reflect(&rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        };
        (self.albedo, scattered)
    }
}
