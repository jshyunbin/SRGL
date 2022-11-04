use srgl::{CanvasBuilder, Color};
use srgl::srt::light::Light;
use srgl::srt::objects::{Objects, Surface};

fn main() -> Result<(), srgl::Error> {
    CanvasBuilder::new()
        .with_title(String::from("simple-window"))
        .with_size(600, 600)
        .with_object(Objects::make_sphere(0.8, 0.2, -7., 0.4, Surface::SHINY))
        .with_object(Objects::make_sphere(-0.8, 0.2, -7., 0.8, Surface::SHINY))
        .with_light(Light::new(8., 8., 3., Color::rgb(178, 178, 178)))
        .set_background(Color::rgb(155, 255, 255))
        .with_rt()
        .build().run()
}