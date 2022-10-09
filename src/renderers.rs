// pub use pixels;

pub mod s2d;
pub mod s3d;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }

    pub fn from(color: [u8; 4]) -> Self {
        Self::new(color[0], color[1], color[2], color[3])
    }

    pub fn to_array(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

pub enum Renderer {
    S2D(s2d::S2D),
    S3D(s3d::S3D),
}

pub trait Render {
    fn render(&self, screen: &mut [u8]);
}

impl Render for Renderer {
    fn render(&self, screen: &mut [u8]) {
        match self {
            Renderer::S2D(s2d) => s2d.render(screen),
            Renderer::S3D(s3d) => s3d.render(screen),
        };
    }
}
