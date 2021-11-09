use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub},
};

use cgmath::{prelude::*, AbsDiffEq, Vector3};
use image::Rgb;
use rand::{distributions::Uniform, prelude::*};

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vec3(pub Vector3<f64>);

impl Vec3 {
    pub fn new<T: Into<f64>>(x: T, y: T, z: T) -> Self {
        Self(Vector3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        })
    }

    pub fn zero() -> Self {
        Self::new(0f64, 0f64, 0f64)
    }

    pub fn random() -> Self {
        Self::new(crate::rand_f64!(), crate::rand_f64!(), crate::rand_f64!())
    }

    #[inline]
    pub fn random_with_range(min: f64, max: f64) -> Self {
        let u = Uniform::new(min, max);
        let mut r = rand::thread_rng();
        Self::new(u.sample(&mut r), u.sample(&mut r), u.sample(&mut r))
    }

    #[inline]
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_with_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    #[inline]
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vec()
    }

    #[inline]
    pub fn random_in_hemisphere(normal: Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal.0).is_sign_positive() {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.magnitude2()
    }

    pub fn length(&self) -> f64 {
        self.magnitude()
    }

    pub fn unit_vec(&self) -> Self {
        self / self.length()
    }

    pub fn is_near_zero(&self) -> bool {
        self.0.abs_diff_eq(&Zero::zero(), 1e-8)
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        self - &(2.0 * self.dot(normal.0) * normal)
    }

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

impl Deref for Vec3 {
    type Target = Vector3<f64>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vec3 {
    #[inline]
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        &mut self.0
    }
}

// Add

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

// Sub

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0)
    }
}

// Mul

impl<T> Mul<T> for Vec3
where
    T: Into<f64>,
{
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs.into())
    }
}

impl<T> Mul<T> for &Vec3
where
    T: Into<f64>,
{
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Vec3(self.0 * rhs.into())
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T> MulAssign<T> for Vec3
where
    T: Into<f64>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs.into()
    }
}

impl MulAssign for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

//Div

impl<T> Div<T> for Vec3
where
    T: Into<f64>,
{
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs.into())
    }
}

impl<T> Div<T> for &Vec3
where
    T: Into<f64>,
{
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Vec3(self.0 / rhs.into())
    }
}

impl<T> DivAssign<T> for Vec3
where
    T: Into<f64>,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs.into()
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

impl Debug for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Zero for Vec3 {
    #[inline]
    fn zero() -> Self {
        Self::zero()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}
