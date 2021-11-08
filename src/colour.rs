use std::ops::{Add, Deref, DerefMut};

use image::Rgb;

use crate::Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
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

impl Add for Colour {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl cgmath::Zero for Colour {
    fn zero() -> Self {
        Self(Vec3::zero())
    }

    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}
