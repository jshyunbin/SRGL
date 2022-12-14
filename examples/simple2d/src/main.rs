use srgl::CanvasBuilder;
use srgl::Shape;
use srgl::Color;

fn main() -> Result<(), srgl::Error> {
    CanvasBuilder::new()
        .with_title(String::from("simple2d"))
        .with_size(400, 300)
        .with_s2d(Shape::make_rect(200., 150.,300., 250.,
                                   Color::from([0x48, 0xb2, 0xe8, 0xff])))
        .build().run()
}