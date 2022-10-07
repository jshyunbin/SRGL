use pixels::wgpu::Color;
use crate::Render;

struct S3D {
    pixels: [[Color]],
}

impl Render for S3D {
    fn render<F>(&self) -> F where F: 'static + FnMut(&mut [u8]) {
        |screen| {
            for (i, pixel) in screen.chunks_exact_mut(4).enumerate() {
                let x = (i % WIDTH as usize) as i16;
                let y = (i / WIDTH as usize) as i16;

                pixel.copy_from_slice(&self.pixels[x][y]);
            }
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