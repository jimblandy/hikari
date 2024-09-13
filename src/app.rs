use crate::window::Window;
use anyhow::Result;
use std::collections::HashMap;
use winit::{event as We, event_loop as Wl, window as Ww};

pub trait App {
    type Window: Window;

    fn create_first_window(&mut self, event_loop: &Wl::ActiveEventLoop) -> Result<Self::Window>;
}

struct Handler<A: App> {
    app: A,
    windows: HashMap<Ww::WindowId, A::Window>,
    result: Result<()>,
}

impl<A: App> winit::application::ApplicationHandler for Handler<A> {
    fn resumed(&mut self, event_loop: &Wl::ActiveEventLoop) {
        if self.windows.is_empty() {
            match self.app.create_first_window(event_loop) {
                Ok(window) => {
                    let id = window.id();
                    let prior = self.windows.insert(id, window);
                    assert!(prior.is_none(), "New window has same id as extant window");
                }
                Err(err) => {
                    self.result = Err(err);
                }
            }
        }
        if self.result.is_err() {
            event_loop.exit();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &Wl::ActiveEventLoop,
        window_id: Ww::WindowId,
        event: We::WindowEvent,
    ) {
        let window = self
            .windows
            .get_mut(&window_id)
            .expect("received event for missing window");
        match event {
            We::WindowEvent::Resized(size) => window.resized(event_loop, size),
            We::WindowEvent::CloseRequested => window.close_requested(event_loop),
            We::WindowEvent::Destroyed => {
                let window = self.windows.remove(&window_id).unwrap();
                window.destroyed(event_loop)
            }
            We::WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => window.keyboard_input(event_loop, device_id, event, is_synthetic),
            We::WindowEvent::ModifiersChanged(modifiers) => {
                window.modifiers_changed(event_loop, modifiers)
            }
            #[allow(deprecated)]
            We::WindowEvent::CursorMoved {
                device_id,
                position,
            } => window.cursor_moved(event_loop, device_id, position),
            We::WindowEvent::Touch(touch) => window.touch(event_loop, touch),
            We::WindowEvent::RedrawRequested => {
                if let Err(err) = window.redraw(event_loop) {
                    self.result = Err(err);
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &Wl::ActiveEventLoop) {
        if self.windows.is_empty() {
            event_loop.exit();
        }
    }

    fn exiting(&mut self, event_loop: &Wl::ActiveEventLoop) {
        for (_, window) in self.windows.drain() {
            let _ = window.destroyed(event_loop);
        }
    }
}

pub fn run<A>(app: A) -> Result<()>
where
    A: App + 'static,
{
    // This doesn't make sense. Surely the app wants to be able to
    // create new windows dynamically; how can it do so and register
    // them with the event loop? And surely the app wants access to
    // its own table of windows.
    let mut handler = Handler {
        app,
        windows: HashMap::new(),
        result: Ok(()),
    };
    Wl::EventLoop::new().unwrap().run_app(&mut handler)?;
    handler.result
}
