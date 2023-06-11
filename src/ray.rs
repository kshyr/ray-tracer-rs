#![allow(non_snake_case)]

use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub A: Vec3,
    pub B: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { A: a, B: b }
    }

    pub fn point_at_parameter(self: Self, t: f32) -> Vec3 {
        self.A + self.B * t
    }
}
