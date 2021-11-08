use std::ops::{Add, AddAssign, Deref, DerefMut};

use image::Rgb;

use crate::Vec3;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Colour(pub Vec3);

impl Colour {
    pub fn to_pixel(self, num_samples: usize) -> Rgb<u8> {
        let scale = 1.0 / num_samples as f64;
        let r = (self.x * scale).sqrt();
        let g = (self.y * scale).sqrt();
        let b = (self.z * scale).sqrt();

        let r = (r.clamp(0.0, 0.999) * 256.0) as u8;
        let g = (g.clamp(0.0, 0.999) * 256.0) as u8;
        let b = (b.clamp(0.0, 0.999) * 256.0) as u8;

        Rgb([r, g, b])
    }
}

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

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
