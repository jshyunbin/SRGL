
use crate::renderers::Render;
pub use crate::renderers::s2d::shape::Shape;
use crate::renderers::Color;

mod shape;

// todo: set pointers to input handler
pub struct S2D {
    shape: Shape,
    width: u32,
    height: u32,
    time: u32,
    background: Color,
}

impl Render for S2D {
    fn render(&self, screen: &mut [u8]) {
        let mut pixels = vec![vec![self.background; self.height as usize]; self.width as usize];

        self.shape.draw(&mut pixels, self.width as usize);

        for (i, pixel) in screen.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize);
            let y = (i / self.width as usize);

            pixel.copy_from_slice(&pixels[x][y].to_array());
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
            background: Color::from([0xff, 0xff, 0xff, 0xff]),
        }
    }
}