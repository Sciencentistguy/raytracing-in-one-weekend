use crate::ray::Ray;

use super::{HitRecord, Hittable};

pub struct HittableList(pub Vec<Hittable>);

impl HittableList {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;
        for obj in &self.0 {
            if let Some(hit) = obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                rec = Some(hit);
            }
        }
        rec
    }
}
