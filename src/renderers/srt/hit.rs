use nalgebra::Vector3;
use crate::srt::objects::Surface;

pub struct Hit {
    position: Vector3<f64>,
    normal: Vector3<f64>,
    surface: Surface,
    object_index: u32,
    t: f64,
}

impl Hit {

}