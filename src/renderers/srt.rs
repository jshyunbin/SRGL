
use crate::renderers::Color;
use crate::srt::objects::Objects;

mod objects;
mod ray;
mod light;


pub struct SRT {
    objects: Vec<Objects>,
    width: u32,
    height: u32,
    background: Color,
}

impl SRT {
    pub fn new(width: u32, height: u32, objects: Vec<Objects>) -> Self {
        Self {
            objects,
            width,
            height,
            background: Color::from([0xff, 0xff, 0xff, 0xff]),
        }
    }


    pub fn render(&self, screen: &mut [u8]) {
        let mut pixels = vec![vec![self.background; self.height as usize]; self.width as usize];


        for (i, pixel) in screen.chunks_exact_mut(4).enumerate() {
            let x = i % self.width as usize;
            let y = i / self.width as usize;

            pixel.copy_from_slice(&pixels[x][y].to_array());
            // time += 1;
        }
    }
}