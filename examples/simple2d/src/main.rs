use srgl::CanvasBuilder;

fn main() -> Result<(), srgl::Error> {
    // let window = Window::setup(RenderSetting::S2D);
    let canvas = CanvasBuilder::new()
        .with_title(String::from("simple2d"))
        .with_size(400, 300)
        .build();

    canvas.run()
}