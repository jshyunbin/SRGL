use std::f64::consts::PI;
use nalgebra::{vector, Vector3};
use crate::renderers::Color;
use crate::srt::objects::{Objects, SphereObj, Surface};
use crate::srt::ray::Ray;
use crate::srt::light::Light;

pub mod objects;
pub mod ray;
pub mod light;


pub struct SRT {
    objects: Vec<Objects>,
    lights: Vec<Light>,
    width: u32,
    height: u32,
    background: Color,
    eye: Vector3<f64>,
    fov: f64,
    uvw: [Vector3<f64>; 3],
}

impl SRT {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            objects: vec![],
            lights: vec![],
            width,
            height,
            background: Color::from([0xff, 0xff, 0xff, 0xff]),
            eye: vector![0., 0., 0.],
            fov: 60.,
            uvw: [vector![1., 0., 0.], vector![0., 1., 0.], vector![0., 0., 1.]],
        }
    }

    // only for debugging; remove for release
    pub fn example(width: u32, height: u32) -> Self {
        Self {
            objects: vec![Objects::make_sphere(0.8, 0.2, -7., 0.4, Surface::SHINY),
                          Objects::make_sphere(-0.8, 0.2, -7., 0.4, Surface::SHINY)],
            lights: vec![Light::new(8., 8., 3., Color::rgb(178, 178, 178))],
            width,
            height,
            background: Color::rgb(155, 255, 255),
            eye: vector![0., 0., 0.],
            fov: 60.,
            uvw: [vector![1., 0., 0.], vector![0., 1., 0.], vector![0., 0., 1.]],
        }
    }

    pub fn set_eye(&mut self, x: f64, y: f64, z: f64) {
        self.eye = vector![x, y, z];
    }

    pub fn set_fov(&mut self, fov: f64) {
        self.fov = fov;
    }

    pub fn set_uvw(&mut self, u: [f64; 3], v: [f64; 3], w: [f64; 3]) {
        self.uvw = [Vector3::from(u), Vector3::from(v), Vector3::from(w)];
    }

    pub fn add_object(&mut self, object: Objects) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light)
    }

    pub fn render(&self, screen: &mut [u8]) {

        for (i, pixel) in screen.chunks_exact_mut(4).enumerate() {
            let x = i % self.width as usize;
            let y = i / self.width as usize;
            let y = self.height as usize - y - 1;

            // calculate view rays here
            let u = 2. * x as f64 / self.width as f64 - 1.;
            let v = 2. * y as f64 / self.height as f64 - 1.;
            let d = 2. / (self.fov * PI / 360.).tan();

            let mut eye_ray = vector![0., 0., 0.];

            eye_ray += self.uvw[0] * u;
            eye_ray += self.uvw[1] * v;
            eye_ray += self.uvw[2] * -d;

            let eye_ray = Ray::from_vector(self.eye, eye_ray);


            pixel.copy_from_slice(&eye_ray.get_color(&self.objects, &self.lights, 1)
                .unwrap_or(self.background).to_array());
        }
    }
}