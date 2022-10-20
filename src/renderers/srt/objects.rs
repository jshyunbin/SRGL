use nalgebra::Vector3;
use crate::{Color, Shape};
use crate::srt::ray::Ray;


pub struct Surface {
    diffuse: Vector3<f64>,
    ambient: Vector3<f64>,
    specular: Vector3<f64>,
    spec_power: f64,
    k_refl: f64,
}

impl Surface {
    // pub fn new(diffuse: (f64, f64, f64), ambient: (f64, f64, f64), specular: (f64, f64, f64),
    //         spec_power: f64, k_refl: f64) -> Self {
    //     Self {
    //         diffuse: Vector3::new(*diffuse),
    //         ambient: Vector3::new()
    //
    //     }
    // }

    pub const SHINY: Self = Self {
        diffuse: Vector3::new(0.6, 0.6, 0.6),
        ambient: Vector3::new(0.2, 0.2, 0.2),
        specular: Vector3::new(0.7, 0.7, 0.7),
        spec_power: 20.,
        k_refl: 0.7,
    };

    pub const MATTE_RED: Self = Self {
        diffuse: Vector3::new(0.4, 0.1, 0.1),
        ambient: Vector3::new(0.3, 0., 0.),
        specular: Vector3::new(0., 0., 0.),
        spec_power: 1.,
        k_refl: 0.,
    };
}

pub enum Objects {
    Sphere(SphereObj),
    Triangle(TriangleObj),
    Cylinder(CylinderObj),
    Cone(ConeObj),
    Box(BoxObj),
}

impl Objects {
    pub fn make_sphere(x: f64, y: f64, z: f64, r: f64, surface: Surface) -> Self {
        Self::Sphere(SphereObj{
            position: Vector3::new(x, y, z),
            radius: r,
            surface,
        })
    }

    pub fn hit(&self, ray: &Ray) -> Option<(f64, f64)> {
        match self {
            Self::Sphere(sphere) => {
                let a = ray.get_direction().norm_squared();
                let b = 2. * ray.get_direction().dot(&(ray.get_start_pos() - sphere.position));
                let c = (ray.get_start_pos() - sphere.position).norm_squared() - sphere.radius.powi(2);
                let d = b * b - 4. * a * c;
                if d < 0. {
                    None
                } else {
                    let d = d.sqrt();
                    let r1= (-b-d)/(2.*a);
                    let r2= (-b+d)/(2.*a);
                    Some((r1, r2))
                }
            },
            Self::Triangle(triangle) => None,
            Self::Cylinder(cylinder) => None,
            Self::Cone(cone) => None,
            Self::Box(box_) => None,
        }
    }
}

pub struct SphereObj {
    position: Vector3<f64>,
    radius: f64,
    surface: Surface,
}

pub struct TriangleObj {

}

pub struct CylinderObj {

}

pub struct ConeObj {

}

pub struct BoxObj {

}