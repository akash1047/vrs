use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let _window = WindowBuilder::new()
        .with_title("vrs")
        .with_inner_size(LogicalSize::new(800, 600))
        .build(&event_loop);

    event_loop.set_control_flow(ControlFlow::Poll);

    let _ = event_loop.run(move |event, _elwt| match event {
        _ => {}
    });
}
