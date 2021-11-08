use crate::{point::Point, ray::Ray, Vec3, ASPECT_RATIO};

pub struct Camera {
    pub origin: Point,
    pub lower_left_corner: Point,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ASPECT_RATIO;
        let focal_length = 1.0;

        let origin = Point(Vec3::zero());

        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            Point(origin.0 - horizontal / 2 - vertical / 2 - Vec3::new(0.0, 0.0, focal_length));

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    #[inline]
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner.0 + u * self.horizontal + v * self.vertical
                - self.origin.0,
        }
    }
}
