use cgmath::{Angle, Deg, Rad};

use crate::{ray::Ray, Vec3};

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(
        position: Vec3,
        target: Vec3,
        vert: Vec3,
        vfov: Deg<f64>,
        aspect_ratio: f64,
    ) -> Self {
        let theta = Rad::from(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (position - target).unit_vec();
        let u: Vec3 = vert.cross(w.0).into();
        let v: Vec3 = w.cross(u.0).into();

        let focal_length = 1.0;

        let origin = position;

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2 - vertical / 2 - w;

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
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
