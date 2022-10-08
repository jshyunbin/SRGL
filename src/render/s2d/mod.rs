
use crate::Rend;
use pixels::wgpu::Color;

enum Shape {

}

pub struct S2D {
    shapes: Vec<Shape>,
    width: u32,
    height: u32,
}

impl Rend for S2D {
    fn render(&self, screen: &mut [u8]) {
        for (i, pixel) in screen.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize) as i16;
            let y = (i / self.width as usize) as i16;

            pixel.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff]);
        }
    }
}

impl S2D {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            shapes: vec![],
            width: width,
            height: height,
        }
    }
}