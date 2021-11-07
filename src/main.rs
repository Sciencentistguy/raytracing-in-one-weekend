mod colour;
mod math;
mod point;
mod ray;

use core::slice;
use std::ops::{Deref, DerefMut};

use crate::colour::Colour;

#[allow(unused_imports)]
use cgmath::prelude::*;

use image::{ImageBuffer, Pixel, Rgb, RgbImage};
pub use math::Vec3;
use point::Point;
use ray::Ray;
use static_assertions::const_assert_eq;

const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = 1080;
const IMAGE_ASPECT_RATIO: f64 = 1.777_777_777_777_777_7;

fn main() {
    let mut image = RgbImage::new(IMAGE_HEIGHT, IMAGE_WIDTH);

    let viewport_height = 2f64;
    let viewport_width = 2f64 * IMAGE_ASPECT_RATIO;
    let focal_length = 1f64;

    let origin = Point(Vec3::zero());
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin.0 - horizontal / 2 - vertical / 2 - Vec3::new(0.0, 0.0, focal_length);

    for (_, row) in image.enumerate_rows_mut() {
        for (x, y, pix) in row {
            let u = x as f64 / (IMAGE_WIDTH as f64 - 1f64);
            let v = y as f64 / (IMAGE_HEIGHT as f64 - 1f64);
            let r = Ray {
                orig: origin,
                dir: lower_left_corner + u * horizontal + v * vertical - origin.0,
            };
            let colour = ray_colour(&r);
            *pix = colour.into();
        }
    }

    // Why is it upside down? no idea, flip it
    image_as_mut_slice(&mut image).reverse();

    image.save("image.png").unwrap();
}

fn ray_colour(ray: &Ray) -> Colour {
    let unit_dir = ray.dir.unit_vec();
    let t = 0.5 * (unit_dir.y + 1.0);
    Colour((1.0 - t) * Vec3::new(1, 1, 1) + t * Vec3::new(0.5, 0.7, 1.0))
}

/// Get a `&mut [u8]` from an RgbImage
fn image_as_mut_slice(img: &mut RgbImage) -> &mut [Rgb<u8>] {
    let len = img.len();
    // A pointer to the start of the slice, typed as u8
    let p = img.as_mut_ptr();
    // A pointer to the start of the slice, but as Rgb
    let ap: *mut Rgb<u8> = p.cast();

    const_assert_eq!(
        std::mem::size_of::<Rgb<u8>>(),
        3 * std::mem::size_of::<u8>()
    );

    debug_assert!(len % 3 == 0);

    // Safety:
    // Creating a mut ref from a mut ref is allowed
    // the length is correct due to the const_assert, the debug_assert should be impossible to trip
    unsafe { slice::from_raw_parts_mut(ap, len / 3) }
}
