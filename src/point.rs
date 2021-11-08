use std::ops::Deref;
use std::ops::DerefMut;

use crate::Vec3;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Point(pub Vec3);

impl Deref for Point {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Point {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
