use nalgebra::{Matrix, SVector};
use pixels::wgpu::Color;


pub enum Shape {
    Point(PointShape),
    Line(LineShape),
    Circle(CircleShape),
    Rect(RectShape),
    Vertices(ClosedShape),
    Shapes(Vec<Shape>),
}


impl Shape {
    pub fn add(mut self, shape: Shape) -> Self {
        match self {
            Shape::Shapes(shapes) => Shape::Shapes(shapes.push(shape)),
            s => Shape::Shapes(vec![s, shape]),
        }
    }
}

struct PointShape {
    coord: SVector<f64, 3>,
    point_color: Color,
    size: f64,
}

struct LineShape {
    start: SVector<f64, 3>,
    end: SVector<f64, 3>,
    stroke_color: Color,
    stroke_weight: f64,
}

struct CircleShape {
    coord: SVector<f64, 3>,
    radius: f64,
    color: Color,
    stroke: bool,
    stroke_color: Color,
    stroke_weight: f64,
}

struct RectShape {
    start: SVector<f64, 3>,
    end: SVector<f64, 3>,
    color: Color,
    stroke: bool,
    stroke_color: Color,
    stroke_weight: f64,
}

struct ClosedShape {
    vertices: SVector<f64, 3>,
    color: Color,
    stroke: bool,
    stroke_color: Color,
    stroke_weight: f64,
}