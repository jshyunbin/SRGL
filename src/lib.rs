
pub use crate::s2d::*;
pub use crate::s3d::*;
pub use pixels::Error;
use pixels::{Pixels, SurfaceTexture};
use pixels::Error::Surface;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;


mod s2d;
mod s3d;

pub enum RenderSetting {
    S2D,
    S3D(s3d::Camera, s3d::Shader, s3d::RenderMethod)
}

pub struct Window {
    render_setting: RenderSetting,
}

impl Window {
    pub fn setup(render_setting: RenderSetting) -> Self {
        Self {
            render_setting: render_setting
        }
    }

    pub fn matrix_push() {

    }

    pub fn matrix_pop() {

    }

    pub fn run(width: u32, height: u32) -> Result<(), Error> {

        let size = LogicalSize::new(width as f64, height as f64);

        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();
        let window = WindowBuilder::new()
            .with_title("simple2d")
            .with_resizable(false)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap();
        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(width, height, surface_texture)?
        };

        event_loop.run(move |event, _, control_flow| {
            if let Event::RedrawRequested(_) = event {
                for (i, pixel) in pixels.get_frame()
                    .chunks_exact_mut(4)
                    .enumerate() {
                    let x = (i % height as usize) as i16;
                    let y = (i / width as usize) as i16;

                    let rgba = [0x48, 0xb2, 0xe8, 0xff];
                    pixel.copy_from_slice(&rgba);
                }
                if pixels
                    .render()
                    .map_err(|e| panic!("pixels.render() failed: {e}"))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Handle input events
            if input.update(&event) {
                // Close events
                if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                if let Some(size) = input.window_resized() {
                    pixels.resize_surface(size.width, size.height);
                }

                // Update internal state and request a redraw
                window.request_redraw();
            }
        });
    }
}