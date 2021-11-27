use crate::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn at(&self, ray_parameter: f64) -> Vec3 {
        self.origin + ray_parameter * self.direction
    }
}
