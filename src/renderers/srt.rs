use nalgebra::Vector3;
use crate::renderers::Color;
use crate::srt::objects::Objects;

mod objects;
mod ray;
mod light;


pub struct SRT {
    objects: Vec<Objects>,
    width: u32,
    height: u32,
    background: Color,
    eye: Vector3<f64>,
    fov: f64,
    uvw: [Vector3<f64>; 3],
}

impl SRT {
    pub fn new(width: u32, height: u32, objects: Vec<Objects>) -> Self {
        Self {
            objects,
            width,
            height,
            background: Color::from([0xff, 0xff, 0xff, 0xff]),
            eye: Vector3::new(0., 0., 0.),
            fov: 60.,
            uvw: [Vector3::new(1., 0., 0.), Vector3::new(0., 1., 0.),
                    Vector3::new(0., 0., 1.)],
        }
    }

    pub fn set_eye(&mut self, x: f64, y: f64, z: f64) {
        self.eye = Vector3::new(x, y, z);
    }

    pub fn set_fov(&mut self, fov: f64) {
        self.fov = fov;
    }

    pub fn set_uvw(&mut self, u: [f64; 3], v: [f64; 3], w: [f64; 3]) {
        self.uvw = [Vector3::from(u), Vector3::from(v), Vector3::from(w)];
    }


    pub fn render(&self, screen: &mut [u8]) {

        for (i, pixel) in screen.chunks_exact_mut(4).enumerate() {
            let x = i % self.width as usize;
            let y = i / self.width as usize;

            // calculate view rays here

            pixel.copy_from_slice(&self.background.to_array());
            // time += 1;
        }
    }
}