use crate::{point::Point, Vec3};

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, ray_parameter: f64) -> Point {
        Point(self.origin.0 + ray_parameter * self.direction)
    }
}
