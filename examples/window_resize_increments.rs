use log::debug;
use simple_logger::SimpleLogger;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::Key,
    window::WindowBuilder,
};

#[path = "util/fill.rs"]
mod fill;

fn main() -> Result<(), impl std::error::Error> {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(LogicalSize::new(128.0, 128.0))
        .with_resize_increments(LogicalSize::new(25.0, 25.0))
        .build(&event_loop)
        .unwrap();

    let mut has_increments = true;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Space,
                                state: ElementState::Released,
                                ..
                            },
                        ..
                    },
                window_id,
            } if window_id == window.id() => {
                has_increments = !has_increments;

                let new_increments = match window.resize_increments() {
                    Some(_) => None,
                    None => Some(LogicalSize::new(25.0, 25.0)),
                };
                debug!("Had increments: {}", new_increments.is_none());
                window.set_resize_increments(new_increments);
            }
            Event::AboutToWait => window.request_redraw(),
            Event::RedrawRequested(_) => {
                fill::fill_window(&window);
            }
            _ => (),
        }
    })
}
