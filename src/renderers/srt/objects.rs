use nalgebra::{vector, Vector3};
use crate::{Color, Shape};
use crate::srt::ray::Ray;


pub struct Surface {
    pub diffuse: Vector3<f64>,
    pub ambient: Vector3<f64>,
    pub specular: Vector3<f64>,
    pub spec_power: f64,
    pub k_refl: f64,
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

    pub fn copy(&self) -> Self {
        Self {
            diffuse: self.diffuse.xyz(),
            ambient: self.ambient.xyz(),
            specular: self.specular.xyz(),
            spec_power: self.spec_power,
            k_refl: self.k_refl,
        }
    }

    pub const SHINY: Self = Self {
        diffuse: Vector3::new(0.6, 0.6, 0.6),
        ambient: Vector3::new(0.1, 0.1, 0.1),
        specular: Vector3::new(0.7, 0.7, 0.7),
        spec_power: 20.,
        k_refl: 0.7,
    };

    pub const SHINY_GREEN: Self = Self {
        diffuse: Vector3::new(0., 0.5, 0.),
        ambient: Vector3::new(0., 0.2, 0.),
        specular: Vector3::new(0.7, 0.7, 0.7),
        spec_power: 20.,
        k_refl: 0.,
    };

    pub const MATTE_RED: Self = Self {
        diffuse: Vector3::new(0.5, 0., 0.),
        ambient: Vector3::new(0.2, 0., 0.),
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

    // return t value of first hit position and normalized normal vector
    //
    pub fn hit(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)> {
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
                    if r1 > 0. {
                        let pos = ray.get_t_pos(r1);
                        Some((r1, (pos - sphere.position).normalize()))
                    } else {
                        None
                    }
                }
            },
            Self::Triangle(triangle) => None,
            Self::Cylinder(cylinder) => None,
            Self::Cone(cone) => None,
            Self::Box(box_) => None,
        }
    }

    pub fn get_surface(&self) -> Surface {
        match self {
            Self::Sphere(sphere) => sphere.surface.copy(),
            Self::Triangle(triangle) => Surface::SHINY,
            Self::Cylinder(cylinder) => Surface::SHINY,
            Self::Cone(cone) => Surface::SHINY,
            Self::Box(box_) => Surface::SHINY,
        }
    }

    pub fn get_ambient(&self) -> Vector3<f64> {
        match self {
            Self::Sphere(sphere) => sphere.surface.ambient,
            Self::Triangle(triangle) => vector![0., 0., 0.],
            Self::Cylinder(cylinder) => vector![0., 0., 0.],
            Self::Cone(cone) => vector![0., 0., 0.],
            Self::Box(box_) => vector![0., 0., 0.],
        }
    }

    pub fn get_diffuse(&self) -> Vector3<f64> {
        match self {
            Self::Sphere(sphere) => sphere.surface.diffuse,
            Self::Triangle(triangle) => vector![0., 0., 0.],
            Self::Cylinder(cylinder) => vector![0., 0., 0.],
            Self::Cone(cone) => vector![0., 0., 0.],
            Self::Box(box_) => vector![0., 0., 0.],
        }
    }

    pub fn get_specular(&self) -> Vector3<f64> {
        match self {
            Self::Sphere(sphere) => sphere.surface.specular,
            Self::Triangle(triangle) => vector![0., 0., 0.],
            Self::Cylinder(cylinder) => vector![0., 0., 0.],
            Self::Cone(cone) => vector![0., 0., 0.],
            Self::Box(box_) => vector![0., 0., 0.],
        }
    }

    pub fn get_spec_power(&self) -> f64 {
        match self {
            Self::Sphere(sphere) => sphere.surface.spec_power,
            Self::Triangle(triangle) => 0.,
            Self::Cylinder(cylinder) => 0.,
            Self::Cone(cone) => 0.,
            Self::Box(box_) => 0.,
        }
    }

    pub fn get_k_refl(&self) -> f64 {
        match self {
            Self::Sphere(sphere) => sphere.surface.k_refl,
            Self::Triangle(triangle) => 0.,
            Self::Cylinder(cylinder) => 0.,
            Self::Cone(cone) => 0.,
            Self::Box(box_) => 0.,
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