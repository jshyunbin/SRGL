use nalgebra::{vector, Vector3};
use crate::Color;
use crate::srt::objects::Objects;
use crate::srt::light::Light;

pub struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>
}

impl Ray {
    pub fn new(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Self {
        Self {
            origin: Vector3::new(x1, y1, z1),
            direction: Vector3::new(x2 - x1, y2 - y1, z2 - z1),
        }
    }

    pub fn from_vector(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Self {
            origin,
            direction,
        }
    }

    pub fn get_direction(&self) -> Vector3<f64> {
        self.direction
    }

    pub fn get_direction_norm(&self) -> Vector3<f64> {
        self.direction.normalize()
    }

    pub fn get_start_pos(&self) -> Vector3<f64> {
        self.origin
    }

    pub fn get_end_pos(&self) -> Vector3<f64> {
        self.origin + self.direction
    }

    pub fn get_t_pos(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }

    pub fn get_color(&self, objects: &Vec<Objects>, lights: &Vec<Light>, iter: u32) -> Option<Color> {
        if iter == 0 {
            None
        } else {
            let mut mint: f64 = 0.;
            let mut minn: Vector3<f64> = vector![0., 0., 0.];
            let mut closest_obj: Option<&Objects> = None;
            for object in objects {
                let t = object.hit(self);
                if let Some(hitt) = t {
                    let (hitp, hitn) = hitt;
                    if t.is_none() || mint > hitp {
                        mint = hitp;
                        minn = hitn;
                        closest_obj = Some(object);
                    }
                }
            }
            if closest_obj.is_none() {
                None
            } else {
                let closest_obj = closest_obj.unwrap();
                let pos = self.get_t_pos(mint);

                let mut amb: Vector3<f64> = vector![0., 0., 0.];

                let mut dif: Vector3<f64> = vector![0., 0., 0.];
                for l in lights {
                    let cl = l.get_color_vector();
                    let cr = closest_obj.get_diffuse();
                    let ldir = (pos - l.get_position()).normalize();
                    dif += cl.component_mul(&cr) * f64::max(-ldir.dot(&minn), 0.);
                }


                let mut spc: Vector3<f64> = vector![0., 0., 0.];
                for l in lights {
                    let cl = l.get_color_vector();
                    let cp = closest_obj.get_specular();
                    let p = closest_obj.get_spec_power();
                    let ldir = (pos - l.get_position()).normalize();
                    spc += cl.component_mul(&cp) * (ldir - minn).normalize().dot(&minn).powf(p);
                }


                let c: Vector3<f64> = amb + dif + spc;

                Some(Color::from_vector(c))
            }
        }
    }
}