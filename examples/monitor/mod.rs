use torophy::{Space};

use glium::glutin;
use glium::glutin::event::{Event, WindowEvent, DeviceEvent, VirtualKeyCode, ElementState};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::Surface;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::{Instant, Duration};

mod drawing;
mod renderer;

pub struct GraphicMonitor {
    display: glium::Display,
    event_loop: EventLoop<()>,
    renderer: renderer::Renderer,

    imgui: imgui::Context,
    imgui_renderer: Renderer,
    imgui_platform: WinitPlatform,

    space: Space,
}

impl GraphicMonitor {
    pub fn new(space: Space) -> GraphicMonitor {
        let width = space.dimension().width;
        let height = space.dimension().height;

        let window_builder = WindowBuilder::new()
            .with_title("Torophy monitor")
            .with_inner_size(glutin::dpi::LogicalSize::new(width, height));

        let glutin_context = glutin::ContextBuilder::new()
            .with_multisampling(8)
            .with_vsync(true);

        let event_loop = EventLoop::new();
        let display = glium::Display::new(window_builder, glutin_context, &event_loop)
            .expect("Failed to initialize glium display");

        let renderer = renderer::Renderer::new(&display, (width as f32, height as f32));

        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        let mut imgui_platform = WinitPlatform::init(&mut imgui);
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            imgui_platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);
        }

        let hidpi_factor = imgui_platform.hidpi_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.fonts().add_font(&[
            imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    size_pixels: font_size,
                    ..imgui::FontConfig::default()
                }),
            },
        ]);

        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        let imgui_renderer = Renderer::init(&mut imgui, &display)
            .expect("Failed to initialize imgui renderer");

        GraphicMonitor {
            display,
            event_loop,
            renderer,
            imgui,
            imgui_renderer,
            imgui_platform,
            space,
        }
    }

    pub fn main_loop<F>(self, mut frame_action: F)
    where F: 'static + FnMut(&mut Space) {
        let GraphicMonitor { display, event_loop, renderer, mut imgui, mut imgui_renderer, mut imgui_platform, mut space } = self;
        let mut last_frame = Instant::now();
        let mut last_physics_time = Duration::from_secs(0);
        let mut ctrl_key = false;
        event_loop.run(move |event, _, control_flow| {
            let gl_window = display.gl_window();
            match event {
                Event::NewEvents(_) => {
                    last_frame = imgui.io_mut().update_delta_time(last_frame);
                    frame_action(&mut space);
                    last_physics_time = Instant::now() - last_frame;
                },
                Event::MainEventsCleared => {
                    imgui_platform.prepare_frame(imgui.io_mut(), &gl_window.window())
                        .expect("Failed to prepare frame");
                    gl_window.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 0.0, 1.0);

                    drawing::draw_space(&renderer, &mut target, &space);

                    let mut imgui_ui = imgui.frame();
                    drawing::draw_ui(&mut imgui_ui, &space, last_physics_time);
                    imgui_platform.prepare_render(&imgui_ui, gl_window.window());
                    let imgui_draw_data = imgui_ui.render();
                    imgui_renderer.render(&mut target, imgui_draw_data).expect("Rendering failed");

                    target.finish().unwrap();
                }
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = ControlFlow::Exit,
                Event::DeviceEvent { event: DeviceEvent::ModifiersChanged(modifiers), .. } => { ctrl_key = modifiers.ctrl() },
                Event::DeviceEvent { event: DeviceEvent::Key(input), .. } =>  {
                    if input.virtual_keycode == Some(VirtualKeyCode::W)
                        && input.state == ElementState::Pressed
                        && ctrl_key {
                        *control_flow = ControlFlow::Exit
                    }
                }
                event => {
                    imgui_platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
                }
            };
        });
    }
}
