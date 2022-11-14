use std::f64::consts::PI;
use nalgebra::{vector, Vector3};
use crate::Scene;
use crate::renderers::Color;
use crate::srt::objects::{Objects, Surface};
use crate::srt::ray::Ray;
use crate::srt::hit::Hit;
use crate::srt::light::Light;

pub mod objects;
pub mod ray;
pub mod light;
pub mod hit;


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
    pub fn new(width: u32, height: u32, canvas: Scene) -> Self {
        Self {
            objects: canvas.objects,
            lights: canvas.lights,
            width,
            height,
            background: canvas.background,
            eye: canvas.eye,
            fov: canvas.fov,
            uvw: canvas.uvw,
        }
    }

    pub fn get_color(&self, ray: &Ray, iter: u32) -> Color {
        if iter == 0 {
            Color::rgb(0, 0, 0)
        } else {
            let mut min_hit: Hit = Hit::default();
            let mut min_exists: bool = false;
            for i in 0..self.objects.len() {
                let hit = self.objects[i].hit(ray);
                if let Some(hit) = hit {
                    let (hitt, hitn) = hit;
                    if !min_exists || min_hit.t > hitt {
                        min_exists = true;
                        min_hit = Hit::new(hitt, i, hitn);
                    }
                }
            }
            if !min_exists {
                self.background
            } else {
                min_hit.position = ray.get_t_pos(min_hit.t);
                min_hit.surface = self.objects[min_hit.object_index].get_surface();

                let amb: Vector3<f64> = self.objects[min_hit.object_index].get_ambient();

                let mut dif: Vector3<f64> = vector![0., 0., 0.];
                for l in &self.lights {
                    let cl = l.get_color_vector();
                    let cr = min_hit.surface.diffuse;
                    let ldir = (min_hit.position - l.get_position()).normalize();
                    dif += cl.component_mul(&cr) * f64::max(-ldir.dot(&min_hit.normal), 0.);
                }


                let mut spc: Vector3<f64> = vector![0., 0., 0.];
                for l in &self.lights {
                    let cl = l.get_color_vector();
                    let cp = min_hit.surface.specular;
                    let p = min_hit.surface.spec_power;
                    let ldir = (min_hit.position - l.get_position()).normalize();
                    spc += cl.component_mul(&cp) * (ldir - min_hit.normal).normalize()
                        .dot(&min_hit.normal).powf(p);
                }

                let dir = -ray.get_direction();
                let refl = min_hit.normal * (2. * dir.dot(&min_hit.normal)) - dir;
                let refl_color = self.get_color(&Ray::from_vector(min_hit.position, refl), iter - 1);
                let refl_color = refl_color.to_vector();

                let c: Vector3<f64> = amb + dif + spc + min_hit.surface.k_refl * refl_color;

                Color::from_vector(c)
            }
        }
    }

    pub fn render(&self, screen: &mut [u8]) {

        for (i, pixel) in screen.chunks_exact_mut(4).enumerate() {
            let x = i % self.width as usize;
            let y = i / self.width as usize;
            let y = self.height as usize - y - 1;

            // calculate view rays here
            let u = 2. * x as f64 / self.width as f64 - 1.;
            let v = 2. * y as f64 / self.height as f64 - 1.;
            let d = 1. / (self.fov * PI / 360.).tan();

            let mut eye_ray = vector![0., 0., 0.];

            eye_ray += self.uvw[0] * u;
            eye_ray += self.uvw[1] * v;
            eye_ray += self.uvw[2] * -d;

            let eye_ray = Ray::from_vector(self.eye, eye_ray);


            pixel.copy_from_slice(&self.get_color(&eye_ray, 3).to_array());
        }
    }
}