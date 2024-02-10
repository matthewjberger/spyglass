use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .build(&event_loop)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;

        match event {
            winit::event::Event::NewEvents(..) => {
                // Handle events
                // Update
            }

            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }

            winit::event::Event::MainEventsCleared => {
                // Draw
            }

            winit::event::Event::WindowEvent {
                event:
                    winit::event::WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if let (winit::event::VirtualKeyCode::Escape, winit::event::ElementState::Pressed) =
                    (keycode, state)
                {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
            }

            _ => {}
        }
    });
}
