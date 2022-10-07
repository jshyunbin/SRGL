
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