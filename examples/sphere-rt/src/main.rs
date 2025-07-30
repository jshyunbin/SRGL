use srgl::{CanvasBuilder, Color};
use srgl::srt::light::Light;
use srgl::srt::objects::{Objects, Surface};

fn main() -> Result<(), srgl::Error> {
    CanvasBuilder::new()
        .with_title(String::from("sphere raytracing"))
        .with_size(480, 480)
        .set_eye(4., 0., 0.)
        .set_uvw(vec![0., 0., -1.], vec![0., 1., 0.], vec![1., 0., 0.])
        .with_object(Objects::make_sphere(0., 0., 0., 1., Surface::SHINY_GREEN))
        .with_object(Objects::make_sphere(1., 0.6, -1., 0.3, Surface::MATTE_RED))
        .with_light(Light::new(7., 7., -5., Color::rgb(255, 255, 255)))
        .set_background(Color::rgb(102, 102, 230))
        .with_rt()
        .build().run()
}