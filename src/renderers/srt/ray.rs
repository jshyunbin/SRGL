use nalgebra::Vector3;
use crate::Color;
use crate::srt::objects::Objects;

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

    pub fn get_start_pos(&self) -> Vector3<f64> {
        self.origin
    }

    pub fn get_end_pos(&self) -> Vector3<f64> {
        self.origin + self.direction
    }

    pub fn get_t_pos(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }

    pub fn get_color(&self, objects: &Vec<Objects>, iter: u32) -> Color {
        if iter == 0 {
            Color::rgb(0, 0, 0)
        } else {
            let mut pos: Vector3<f64>;
            let mut closest_obj: &Objects;
            for object in objects {
                let t = object.hit(self);

            }
            Color::rgb(0, 0, 0)
        }
    }
}