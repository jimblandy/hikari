use play_wgpu::app;
use winit::{event as We, event_loop::ControlFlow, window as Ww};

fn main() -> anyhow::Result<()> {
    let event_loop = winit::event_loop::EventLoop::new();

    let app = app::App::new(&event_loop)?;

    event_loop.run(move |event, _target, control_flow| {
        let result = match event {
            We::Event::NewEvents(_) => Ok(()),
            We::Event::WindowEvent { window_id, event } => julia.window_event(window_id, event),
            We::Event::DeviceEvent { device_id: _, event: _ } => todo!(),
            We::Event::UserEvent(_) => todo!(),
            We::Event::Suspended => todo!(),
            We::Event::Resumed => todo!(),
            We::Event::MainEventsCleared => todo!(),
            We::Event::RedrawRequested(_) => todo!(),
            We::Event::RedrawEventsCleared => todo!(),
            We::Event::LoopDestroyed => todo!(),
        };

        match result {
            Ok(()) => control_flow.set_wait(),
            Err(c) => {
                *control_flow = c;
            }
        }
    });
}

struct Julia {
    app: app::App,
}

impl Julia {
    fn window_event(
        &mut self,
        window_id: Ww::WindowId,
        event: We::WindowEvent,
    ) -> Result<(), ControlFlow> {
        match event {
            We::WindowEvent::CloseRequested => {
                Err(ControlFlow::Exit)
            }
            We::WindowEvent::Resized(_) => todo!(),
            We::WindowEvent::Destroyed => todo!(),
            We::WindowEvent::ReceivedCharacter(_) => todo!(),
            We::WindowEvent::KeyboardInput {
                device_id,
                input,
                is_synthetic,
            } => self.keyboard_input(window_id, device_id, input, is_synthetic),
            _ => Ok(())
        }
    }

    fn keyboard_input(&self, _window_id: Ww::WindowId, _device_id: We::DeviceId, input: We::KeyboardInput, _is_synthetic: bool) -> Result<(), ControlFlow> {
        match input.virtual_keycode {
            Some(We::VirtualKeyCode::Escape) => Err(ControlFlow::Exit),
            _ => Ok(())
        }
    }
}
