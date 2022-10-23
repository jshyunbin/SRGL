use nalgebra::{vector, Vector, Vector3};
use crate::Color;

pub struct Light {
    position: Vector3<f64>,
    color: Color,
}

impl Light {
    pub fn new(x: f64, y: f64, z: f64, c: Color) -> Self {
        Self {
            position: vector![x, y, z],
            color: c,
        }
    }

    pub fn get_color_vector(&self) -> Vector3<f64> {
        self.color.to_vector()
    }

    pub fn get_position(&self) -> Vector3<f64> {
        self.position
    }
}