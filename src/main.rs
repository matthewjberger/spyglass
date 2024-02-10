fn main() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new()?;
    let _window = winit::window::WindowBuilder::new()
        .with_title("My App")
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .build(&event_loop)?;
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop.run(move |event, elwt| {
        match event {
            winit::event::Event::WindowEvent {
                event:
                    winit::event::WindowEvent::KeyboardInput {
                        event:
                            winit::event::KeyEvent {
                                physical_key: winit::keyboard::PhysicalKey::Code(key_code),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if matches!(key_code, winit::keyboard::KeyCode::Escape) {
                    elwt.exit();
                }
            }

            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            }

            winit::event::Event::AboutToWait => {
                // Update
                // Draw
            }

            _ => (),
        }
    })?;

    Ok(())
}
