// For wgpu (modern GPU API)
mod vertex;
mod camera;
mod cube;
mod state;

use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};
use wgpu::SurfaceError;
use state::State;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = Box::leak(Box::new(WindowBuilder::new().build(&event_loop).unwrap()));

    let mut state = pollster::block_on(State::new(window));

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                        }
                        WindowEvent::KeyboardInput {
                            event: key_event,
                            ..
                        } if key_event.state == ElementState::Pressed && key_event.physical_key == winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape) => {
                            elwt.exit();
                        }
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::RedrawRequested => {
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                Err(SurfaceError::Lost) => state.resize(state.window().inner_size()),
                                Err(SurfaceError::OutOfMemory) => elwt.exit(),
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                state.window().request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}