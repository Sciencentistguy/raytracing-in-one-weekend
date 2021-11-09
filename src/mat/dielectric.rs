use cgmath::InnerSpace;

use crate::{hit::HitRecord, math, rand_f64, ray::Ray, Vec3};

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    #[inline]
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Vec3, Ray) {
        let attenuation = Vec3::one();
        let refraction_ratio = if rec.front_face {
            self.ir.recip()
        } else {
            self.ir
        };
        let unit_dir = ray.direction.unit_vec();

        let cos_theta = (-unit_dir).dot(rec.normal.0).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;

        let direction = if cannot_refract
            || math::shlick_reflectance(cos_theta, refraction_ratio) > rand_f64!()
        {
            unit_dir.reflect(&rec.normal)
        } else {
            unit_dir.refract(&rec.normal, refraction_ratio)
        };

        (
            attenuation,
            Ray {
                origin: rec.p,
                direction,
            },
        )
    }
}
