// pub use pixels;

pub mod s2d;
pub mod s3d;


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
