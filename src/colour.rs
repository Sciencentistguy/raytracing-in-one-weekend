use std::ops::{Deref, DerefMut};

use image::Rgb;

use crate::Vec3;

#[derive(Clone, Copy, PartialEq)]
pub struct Colour(pub Vec3);

impl Deref for Colour {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Colour {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Colour> for Rgb<u8> {
    fn from(val: Colour) -> Self {
        let r = (val.0.x * 255.99) as u8;
        let g = (val.0.y * 255.99) as u8;
        let b = (val.0.z * 255.99) as u8;
        Rgb([r, g, b])
    }
}
