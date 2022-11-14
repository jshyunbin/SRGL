use nalgebra::Vector3;
use crate::srt::objects::Surface;
use crate::vector;

pub struct Hit {
    pub position: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub surface: Surface,
    pub object_index: usize,
    pub t: f64,
}

impl Hit {
    pub fn new(t: f64, object_index: usize, normal: Vector3<f64>) -> Self{
        Self {
            position: vector![0., 0., 0.],
            normal,
            surface: Surface::MATTE_RED,
            object_index,
            t,
        }
    }
}

impl Default for Hit {
    fn default() -> Self {
        Self {
            position: vector![0., 0., 0.],
            normal: vector![0., 0., 0.],
            surface: Surface::MATTE_RED,
            object_index: 0,
            t: 0.,
        }
    }
}