mod colour;
mod math;
mod point;

use crate::colour::Colour;

use cgmath::prelude::*;
use image::RgbImage;
pub use math::Vec3;

const IMAGE_HEIGHT: u32 = 256;
const IMAGE_WIDTH: u32 = 256;

fn main() {
    let mut image = RgbImage::new(IMAGE_HEIGHT, IMAGE_WIDTH);

    for (_, row) in image.enumerate_rows_mut() {
        for (j, i, pix) in row {
            let r = i as f64 / (IMAGE_WIDTH as f64 - 1f64);
            let g = j as f64 / (IMAGE_HEIGHT as f64 - 1f64);
            let b = 0.25f64;
            let colour = Colour(Vec3::new(r, g, b));
            *pix = colour.into();
        }
    }
    image.save("image.png").unwrap();
}
