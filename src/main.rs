use glutin::{
    dpi::LogicalSize,
    event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
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

    let asd = GL::new(&context);

    let mut offset = (0.0, 0.0);
    let mut scale = 2.0;

    asd.program().set_vec2("offset", offset);
    asd.program().set_float("scale", scale);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => {
                context.resize(size);
                asd.resize(size.width as i32, size.height as i32);
            }
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::RedrawRequested(_) => {
            asd.draw();
            context.swap_buffers().unwrap();
        }
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::Key(input) => {
                if input.state == ElementState::Pressed {
                    match input.virtual_keycode {
                        Some(key) => match key {
                            VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                            VirtualKeyCode::Right => offset.0 += scale * 0.05,
                            VirtualKeyCode::Left => offset.0 -= scale * 0.05,
                            VirtualKeyCode::Up => offset.1 += scale * 0.05,
                            VirtualKeyCode::Down => offset.1 -= scale * 0.05,
                            VirtualKeyCode::Q => scale /= 0.9,
                            VirtualKeyCode::W => scale *= 0.9,
                            _ => (),
                        },
                        _ => (),
                    };

                    println!("Offset: {:?}", offset);
                    println!("Scale: {}", scale);

                    asd.program().set_vec2("offset", offset);
                    asd.program().set_float("scale", scale);
                    asd.draw();
                    context.swap_buffers().unwrap();
                }
            }
            _ => (),
        },
        _ => (),
    });
}
