
use crate::Render;
use pixels::wgpu::Color;

enum Shape {

}

struct S2D {
    shapes: Vec<Shape>,
    width: u32,
    height: u32,
    pixels: [[Color]],
}

impl Render for S2D {
    fn render<F>(&self) -> F where F: 'static + FnMut(&mut [u8]) {
        |screen| {
            for (i, pixel) in screen.chunks_exact_mut(4).enumerate() {
                let x = (i % WIDTH as usize) as i16;
                let y = (i / WIDTH as usize) as i16;

                pixel.copy_from_slice(&self.pixels[x][y]);
            }
        }
    }
}