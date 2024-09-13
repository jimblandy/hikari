use anyhow::Result;
use winit::{dpi as Wd, event as We, event_loop as Wl, keyboard as Wk, window as Ww};

use Wl::ActiveEventLoop;

#[allow(unused_variables)]
pub trait Window: Sized {
    /// Return the id of the `winit::Window` that `self` is using.
    fn id(&self) -> Ww::WindowId;

    fn redraw(&mut self, event_loop: &ActiveEventLoop) -> Result<()> { Ok(()) }

    fn cursor_moved(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: We::DeviceId,
        position: Wd::PhysicalPosition<f64>,
    ) { }

    fn touch(
        &mut self,
        event_loop: &ActiveEventLoop,
        touch: We::Touch,
    ) { }

    fn modifiers_changed(
        &mut self,
        event_loop: &ActiveEventLoop,
        state: We::Modifiers,
    ) { }

    fn keyboard_input(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: We::DeviceId,
        input: We::KeyEvent,
        is_synthetic: bool,
    ) {
        self.common_keyboard_input(event_loop, device_id, input, is_synthetic)
    }

    fn resized(
        &mut self,
        event_loop: &ActiveEventLoop,
        size: Wd::PhysicalSize<u32>,
    ) { }

    fn close_requested(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
    }

    fn destroyed(self, event_loop: &ActiveEventLoop) { }

    fn common_keyboard_input(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: We::DeviceId,
        input: We::KeyEvent,
        is_synthetic: bool,
    ) {
        match input.logical_key {
            Wk::Key::Named(Wk::NamedKey::Escape) => {
                event_loop.exit();
            }
            Wk::Key::Character(s) => {
                if s == "q" {
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }
}
