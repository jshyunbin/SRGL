
pub use pixels::Error;
pub use crate::renderers::s2d::Shape;
pub use crate::renderers::*;
use renderers::Renderer;
use renderers::s2d::S2D;
use renderers::s3d::S3D;
use renderers::srt::SRT;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit::window::CursorIcon;
use winit_input_helper::WinitInputHelper;

mod renderers;


pub enum RenderType {
    S2D(Shape),
    S3D,
    SRT,
}

#[derive(Default)]
pub struct CanvasBuilder {
    canvas: CanvasAttributes,
}

pub struct CanvasAttributes {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub title: String,
    pub frame_rate: Option<u32>,
    pub render: RenderType,
}

impl Default for CanvasAttributes {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            title: String::from(""),
            frame_rate: None,
            render: RenderType::S2D(Shape::Shapes(vec![])),
        }
    }
}

impl CanvasBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.canvas.width = Some(width);
        self.canvas.height = Some(height);
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.canvas.title = title;
        self
    }

    pub fn with_s2d(mut self, shape: Shape) -> Self {
        self.canvas.render = RenderType::S2D(shape);
        self
    }

    pub fn with_s3d(mut self) -> Self {
        self.canvas.render = RenderType::S3D;
        self
    }

    pub fn with_rt(mut self) -> Self {
        self.canvas.render = RenderType::SRT;
        self
    }

    pub fn build(self) -> Canvas {
        let w = self.canvas.width.expect("Size must be set");
        let h = self.canvas.height.expect("Size must be set");
        Canvas {
            width: w,
            height: h,
            title: self.canvas.title,
            render: match self.canvas.render {
                RenderType::S2D(shape) => Renderer::S2D(S2D::new(w, h, shape)),
                RenderType::S3D => Renderer::S3D(S3D::new(w, h)),
                RenderType::SRT => Renderer::SRT(SRT::new(w, h)),
            }
        }
    }
}

pub struct Canvas {
    width: u32,
    height: u32,
    title: String,
    render: Renderer,
}

impl Canvas {
    pub fn run(self) -> Result<(), Error> {
        let size = LogicalSize::new(self.width as f64, self.height as f64);

        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();
        let window = WindowBuilder::new()
            .with_title(self.title)
            .with_resizable(false)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap();

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width,
                                                      window_size.height, &window);
            Pixels::new(self.width, self.height, surface_texture)?
        };


        event_loop.run(move |event, _, control_flow| {
            if let Event::RedrawRequested(_) = event {
                self.render.render(pixels.get_frame());
                if pixels
                    .render()
                    .map_err(|e| panic!("pixels.renderers() failed: {e}"))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }


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