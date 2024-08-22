mod state;

use winit::error::EventLoopError;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;
use crate::state::State;

fn main()  {
    env_logger::init();
    pollster::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut state = State::new(&window).await;

    event_loop.run(move |event, control_flow|
        match event {
            Event::WindowEvent { ref event, window_id} if window_id == state.window().id() =>
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => control_flow.exit(),
                        WindowEvent::Resized(new_size) => state.resize(*new_size),
                        WindowEvent::RedrawRequested => {
                            state.update();
                            state.render().unwrap();
                        },
                        _ => {}
                }
        },
        _ => {}
    }).unwrap();
}