
use crate::renderers::Render;
use pixels::wgpu::Color;
pub use crate::renderers::s2d::shape::Shape;

mod shape;

// todo: set pointers to input handler
pub struct S2D {
    shape: Shape,
    width: u32,
    height: u32,
    time: u32,
    background: [u8; 4],
}

impl Render for S2D {
    fn render(&self, screen: &mut [u8]) {
        let mut pixels = vec![self.background; (self.width * self.height) as usize];

        self.shape.draw(&mut pixels, self.width as usize);

        for (i, pixel) in screen.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize);
            let y = (i / self.width as usize);

            pixel.copy_from_slice(&pixels[i]);
            // time += 1;
        }
    }
}

impl S2D {
    pub fn new(width: u32, height: u32, shape: Shape) -> Self {
        Self {
            shape,
            width,
            height,
            time: 0,
            background: [0x48, 0xb2, 0xe8, 0xff],
        }
    }
}