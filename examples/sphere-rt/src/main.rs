use srgl::CanvasBuilder;

fn main() -> Result<(), srgl::Error> {
    CanvasBuilder::new()
        .with_title(String::from("simple-window"))
        .with_size(600, 600)
        .with_rt()
        .build().run()
}