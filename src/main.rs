use glutin::{
    ContextBuilder,
    dpi::LogicalSize,
    event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use gl_util::GL;

mod gl_util;
mod shader;

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Fractals")
        .with_inner_size(LogicalSize::new(800, 800));
    let context = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();
    let context = unsafe { context.make_current().unwrap() };

    let gl = GL::new(&context, "src/raymarching.vert", "src/raymarching.frag");

    gl.program().set_vec2("resolution", (800.0, 800.0));

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => {
                context.resize(size);
                gl.resize(size.width as i32, size.height as i32);
                gl.program().set_vec2("resolution", (size.width as f32, size.height as f32));
            }
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::RedrawRequested(_) => {
            gl.draw();
            context.swap_buffers().unwrap();
        }
        Event::DeviceEvent { event, .. } => if let DeviceEvent::Key(input) = event {
            if input.state == ElementState::Pressed {
                if let Some(key) = input.virtual_keycode {
                    match key {
                        VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                        _ => (),
                    }
                };

                gl.draw();
                context.swap_buffers().unwrap();
            }
        },
        _ => (),
    });
}
