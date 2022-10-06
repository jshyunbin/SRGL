use srgl::Window;
use srgl::RenderSetting;

fn main() -> Result<(), srgl::Error> {
    // let window = Window::setup(RenderSetting::S2D);

    Window::run(320, 240)
}