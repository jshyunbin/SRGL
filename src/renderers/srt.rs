use std::f64::consts::PI;
use nalgebra::{vector, Vector3};
use crate::Scene;
use crate::renderers::Color;
use crate::srt::objects::{Objects, Surface};
use crate::srt::ray::Ray;
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
            let mut mint: f64 = 0.;
            let mut minn: Vector3<f64> = vector![0., 0., 0.];
            let mut closest_obj: Option<&Objects> = None;
            for object in &self.objects {
                let hit = object.hit(ray);
                if let Some(hitt) = hit {
                    let (hitp, hitn) = hitt;
                    if closest_obj.is_none() || mint > hitp {
                        mint = hitp;
                        minn = hitn;
                        closest_obj = Some(&object);
                    }
                }
            }
            if closest_obj.is_none() {
                self.background
            } else {
                let closest_obj = closest_obj.unwrap();
                let pos = ray.get_t_pos(mint);

                let amb: Vector3<f64> = closest_obj.get_ambient();

                let mut dif: Vector3<f64> = vector![0., 0., 0.];
                for l in &self.lights {
                    let cl = l.get_color_vector();
                    let cr = closest_obj.get_diffuse();
                    let ldir = (pos - l.get_position()).normalize();
                    dif += cl.component_mul(&cr) * f64::max(-ldir.dot(&minn), 0.);
                }


                let mut spc: Vector3<f64> = vector![0., 0., 0.];
                for l in &self.lights {
                    let cl = l.get_color_vector();
                    let cp = closest_obj.get_specular();
                    let p = closest_obj.get_spec_power();
                    let ldir = (pos - l.get_position()).normalize();
                    spc += cl.component_mul(&cp) * (ldir - minn).normalize().dot(&minn).powf(p);
                }

                let dir = -ray.get_direction();
                let refl = minn * (2. * dir.dot(&minn)) - dir;
                let refl_color = self.get_color(&Ray::from_vector(pos, refl), iter - 1);
                let refl_color = refl_color.to_vector();

                let c: Vector3<f64> = amb + dif + spc + closest_obj.get_k_refl() * refl_color;

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
            let d = 2. / (self.fov * PI / 360.).tan();

            let mut eye_ray = vector![0., 0., 0.];

            eye_ray += self.uvw[0] * u;
            eye_ray += self.uvw[1] * v;
            eye_ray += self.uvw[2] * -d;

            let eye_ray = Ray::from_vector(self.eye, eye_ray);


            pixel.copy_from_slice(&eye_ray.get_color(&self.objects, &self.lights,
                                                     &self.background.to_vector(), 3)
                .unwrap_or(self.background).to_array());
        }
    }
}