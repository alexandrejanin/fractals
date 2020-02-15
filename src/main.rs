use glutin::{
    ContextBuilder,
    dpi::LogicalSize,
    event::{Event, WindowEvent},
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

    let asd = GL::new(&context);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    context.resize(size);
                    asd.resize(size.width as i32, size.height as i32);
                },
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                asd.draw();
                context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
