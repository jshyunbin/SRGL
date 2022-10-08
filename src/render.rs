// pub use pixels;

pub mod s2d;
pub mod s3d;


pub enum Render {
    S2D(s2d::S2D),
    S3D(s3d::S3D),
}

pub trait Rend {
    fn render(&self, screen: &mut [u8]);
}

impl Rend for Render {
    fn render(&self, screen: &mut [u8]) {
        match self {
            Render::S2D(s2d) => s2d.render(screen),
            Render::S3D(s3d) => s3d.render(screen),
        };
    }
}
