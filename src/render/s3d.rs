use crate::render::Rend;
use pixels::wgpu::Color;

pub struct S3D {
    width: u32,
    height: u32,
}

impl Rend for S3D {
    fn render(&self, screen: &mut [u8]) {
        for (i, pixel) in screen.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize) as i16;
            let y = (i / self.width as usize) as i16;

            pixel.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff]);
        }
    }
}

impl S3D {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
        }
    }
}

struct Point {
    x: f64,
    y: f64,
    z: f64
}

pub struct Camera {
    eye: Point,
    center: Point,
    up: (i32, i32, i32)
}

pub enum Shader {
    Flat,
    Gouraud,
    Phong
}

pub enum RenderMethod {
    Rasterization,
    RayTracing(i32)
}