mod colour;
mod math;
mod point;
mod ray;

use core::slice;

use crate::colour::Colour;

#[allow(unused_imports)]
use cgmath::prelude::*;

use image::{Rgb, RgbImage};
pub use math::Vec3;
use ndarray::Axis;
use point::Point;
use ray::Ray;
use static_assertions::const_assert_eq;

const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = 1080;
const IMAGE_ASPECT_RATIO: f64 = 1.777_777_777_777_777_7;

fn main() {
    let mut pixels =
        ndarray::Array2::<Colour>::zeros((IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize));

    let viewport_height = 2f64;
    let viewport_width = 2f64 * IMAGE_ASPECT_RATIO;
    let focal_length = 1f64;

    let origin = Point(Vec3::zero());
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin.0 - horizontal / 2 - vertical / 2 - Vec3::new(0.0, 0.0, focal_length);

    pixels
        .outer_iter_mut()
        .into_iter()
        .enumerate()
        .for_each(|(row_number, mut row)| {
            for (col_number, pixel) in row.iter_mut().enumerate() {
                let u = row_number as f64 / (IMAGE_WIDTH as f64 - 1f64);
                let v = col_number as f64 / (IMAGE_HEIGHT as f64 - 1f64);
                let r = Ray {
                    orig: origin,
                    dir: lower_left_corner + u * horizontal + v * vertical - origin.0,
                };
                let colour = ray_colour(&r);
                *pixel = colour;
            }
        });

    println!("{:?}", &pixels.len_of(Axis(0)));
    println!("{:?}", &pixels.len_of(Axis(1)));

    let mut image = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pix = pixels.get((i as usize, j as usize)).unwrap();
            image.put_pixel(
                i,
                IMAGE_HEIGHT - 1 - j,
                image::Rgb([
                    (pix.x * 255.999) as u8,
                    (pix.y * 255.999) as u8,
                    (pix.z * 255.999) as u8,
                ]),
            );
        }
    }

    image.save("image.png").unwrap();
}

fn ray_colour(ray: &Ray) -> Colour {
    fn hit_sphere(centre: Point, radius: f64, ray: &Ray) -> bool {
        let oc = ray.orig.0 - centre.0;
        let a = ray.dir.dot(ray.dir.0);
        let b = 2.0 * oc.dot(ray.dir.0);
        let c = oc.dot(oc.0) - radius * radius;
        let disc = b * b - 4f64 * a * c;
        disc > 0.0
    }

    if hit_sphere(Point(Vec3::new(0, 0, -1)), 0.5, ray) {
        return Colour(Vec3::new(1, 0, 0));
    }

    let unit_dir = ray.dir.unit_vec();
    let t = 0.5 * (unit_dir.y + 1.0);
    Colour((1.0 - t) * Vec3::new(1, 1, 1) + t * Vec3::new(0.4, 0.7, 1.0))
}
