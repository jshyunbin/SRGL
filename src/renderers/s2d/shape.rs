use nalgebra::{Matrix, Vector3};
use crate::renderers::Color;

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

    pub fn draw(&self, screen: &mut Vec<Vec<Color>>, width: usize) {

        match self {
            Shape::Point(point) => (),
            Shape::Line(line) => (),
            Shape::Circle(circle) => (),
            Shape::Rect(rect) => {
                for x in rect.start[0] as usize..rect.end[0] as usize {
                    for y in rect.start[1] as usize..rect.end[1] as usize {
                        screen[x][y] = rect.color;
                    }
                }
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
pub struct PointShape {
    coord: Vector3<f64>,
    point_color: Color,
    size: f64,
}

#[derive(Copy, Clone)]
pub struct LineShape {
    start: Vector3<f64>,
    end: Vector3<f64>,
    stroke_color: Color,
    stroke_weight: f64,
}

#[derive(Copy, Clone)]
pub struct CircleShape {
    coord: Vector3<f64>,
    radius: f64,
    color: Color,
    stroke: bool,
    stroke_color: Color,
    stroke_weight: f64,
}

#[derive(Copy, Clone)]
pub struct RectShape {
    start: Vector3<f64>,
    end: Vector3<f64>,
    color: Color,
    stroke: bool,
    stroke_color: Color,
    stroke_weight: f64,
}

#[derive(Copy, Clone)]
pub struct ClosedShape {
    vertices: Vector3<f64>,
    color: Color,
    stroke: bool,
    stroke_color: Color,
    stroke_weight: f64,
}