use crate::{point::Point, Vec3};

pub struct Ray {
    pub orig: Point,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, ray_parameter: f64) -> Point {
        Point(self.orig.0 + ray_parameter * self.dir)
    }
}
