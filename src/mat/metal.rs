use crate::{hit::HitRecord, ray::Ray, Vec3};

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    #[inline(always)]
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Vec3, Ray) {
        let reflected = ray.direction.unit_vec().reflect(&rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            time: ray.time,
        };
        (self.albedo, scattered)
    }
}
