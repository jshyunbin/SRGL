use srgl::CanvasBuilder;

fn main() -> Result<(), srgl::Error> {
    // let window = Window::setup(RenderSetting::S2D);
    let canvas = CanvasBuilder::new()
        .with_title(String::from("simple2d"))
        .with_size(400, 300)
        .build();

    canvas.run(|screen| {
        for (i, pixel) in screen.chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff])
        }
    } )
}