use nalgebra::{Matrix, SVector};
use pixels::wgpu::Color;

#[derive(Clone)]
pub enum Shape {
    Point(PointShape),
    Line(LineShape),
    Circle(CircleShape),
    Rect(RectShape),
    Vertices(ClosedShape),
    Shapes(Vec<Shape>),
}


impl Shape {
    pub fn add(self, shape: Shape) -> Self{
        match self {
            Shape::Shapes(mut shapes) => {
                shapes.push(shape);
                Shape::Shapes(shapes)
            },
            s => {
                Shape::Shapes(vec![s, shape])
            },
        }
    }

    pub fn draw(&self, screen: &mut Vec<[u8; 4]>, width: usize) {

        match self {
            Shape::Point(point) => (),
            Shape::Line(line) => (),
            Shape::Circle(circle) => (),
            Shape::Rect(rect) => {

            },
            Shape::Vertices(vertices) => (),
            Shape::Shapes(shapes) => {
                for shape in shapes {
                    shape.draw(screen, width);
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
struct PointShape {
    coord: SVector<f64, 3>,
    point_color: Color,
    size: f64,
}

#[derive(Copy, Clone)]
struct LineShape {
    start: SVector<f64, 3>,
    end: SVector<f64, 3>,
    stroke_color: Color,
    stroke_weight: f64,
}

#[derive(Copy, Clone)]
struct CircleShape {
    coord: SVector<f64, 3>,
    radius: f64,
    color: Color,
    stroke: bool,
    stroke_color: Color,
    stroke_weight: f64,
}

#[derive(Copy, Clone)]
struct RectShape {
    start: SVector<f64, 3>,
    end: SVector<f64, 3>,
    color: Color,
    stroke: bool,
    stroke_color: Color,
    stroke_weight: f64,
}

#[derive(Copy, Clone)]
struct ClosedShape {
    vertices: SVector<f64, 3>,
    color: Color,
    stroke: bool,
    stroke_color: Color,
    stroke_weight: f64,
}