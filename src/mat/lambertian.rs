use crate::{hit::HitRecord, ray::Ray, Vec3};

#[repr(transparent)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    #[inline(always)]
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Vec3, Ray) {
        //let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        //let mut scatter_direction = (rec.p + rec.normal + Vec3::random_in_unit_sphere()) - rec.p;
        let mut scatter_direction = (rec.p + Vec3::random_in_hemisphere(rec.normal)) - rec.p;

        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray {
            origin: rec.p,
            direction: scatter_direction,
            time: ray.time,
        };
        (self.albedo, scattered)
    }
}
