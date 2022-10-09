use srgl::CanvasBuilder;

fn main() -> Result<(), srgl::Error> {
    CanvasBuilder::new()
        .with_title(String::from("simple-window"))
        .with_size(400, 300)
        .build().run()
}