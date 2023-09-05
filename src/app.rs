use crate::window::Window;
use anyhow::Result;
use std::collections::HashMap;
use winit::{event as We, event_loop as Wl, window as Ww};

pub type LoopTarget = Wl::EventLoopWindowTarget<()>;

pub trait App {
    type Window: Window;
    fn event_loop_init(&mut self, _event_loop: &LoopTarget) -> Result<Option<Wl::ControlFlow>> {
        Ok(None)
    }
    fn create_first_window(&mut self, event_loop: &LoopTarget) -> Result<Self::Window>;
}

pub fn run<A>(mut app: A) -> !
where
    A: App + 'static,
{
    // This doesn't make sense. Surely the app wants to be able to
    // create new windows dynamically; how can it do so and register
    // them with the event loop? And surely the app wants access to
    // its own table of windows.
    let mut windows: HashMap<Ww::WindowId, A::Window> = HashMap::new();
    Wl::EventLoop::new().run(move |event, target, control_flow| {
        log::trace!("event loop: {:#?}", event);
        *control_flow = Wl::ControlFlow::Poll;
        let result = match event {
            We::Event::NewEvents(We::StartCause::Init) => app.event_loop_init(target),
            We::Event::WindowEvent { window_id, event } => {
                let window = windows
                    .get_mut(&window_id)
                    .expect("received event for missing window");
                match event {
                    We::WindowEvent::Resized(size) => window.resized(target, size),
                    We::WindowEvent::CloseRequested => window.close_requested(target),
                    We::WindowEvent::Destroyed => {
                        let window = windows.remove(&window_id).unwrap();
                        window.destroyed(target)
                    }
                    We::WindowEvent::ReceivedCharacter(ch) => window.received_character(target, ch),
                    We::WindowEvent::KeyboardInput {
                        device_id,
                        input,
                        is_synthetic,
                    } => window.keyboard_input(target, device_id, input, is_synthetic),
                    We::WindowEvent::ModifiersChanged(modifiers) => {
                        window.modifiers_changed(target, modifiers)
                    }
                    #[allow(deprecated)]
                    We::WindowEvent::CursorMoved {
                        device_id,
                        position,
                        modifiers: _,
                    } => window.cursor_moved(target, device_id, position),
                    _ => Ok(None),
                }
            }
            We::Event::Resumed => {
                if windows.is_empty() {
                    match app.create_first_window(target) {
                        Ok(window) => {
                            let id = window.id();
                            let prior = windows.insert(id, window);
                            assert!(prior.is_none(), "New window has same id as extant window");
                            Ok(None)
                        }
                        Err(err) => Err(err),
                    }
                } else {
                    Ok(None)
                }
            }
            We::Event::RedrawRequested(window_id) => {
                let window = windows
                    .get_mut(&window_id)
                    .expect("received event for missing window");
                window.redraw(target)
            }
            We::Event::LoopDestroyed => {
                for (_, window) in windows.drain() {
                    let _ = window.destroyed(target);
                }
                Ok(None)
            }
            _ => Ok(None),
        };

        match result {
            Ok(Some(cf)) => {
                *control_flow = cf;
            }
            Ok(None) => {}
            Err(err) => {
                eprintln!("{}", err);
                *control_flow = Wl::ControlFlow::ExitWithCode(1);
            }
        }
    });
}
