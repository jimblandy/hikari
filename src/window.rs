use anyhow::Result;
use winit::{dpi as Wd, event as We, event_loop as Wl, window as Ww};

use crate::app::LoopTarget;

#[allow(unused_variables)]
pub trait Window: Sized {
    /// Return the id of the `winit::Window` that `self` is using.
    fn id(&self) -> Ww::WindowId;

    fn redraw(&mut self, event_loop: &LoopTarget) -> Result<Option<Wl::ControlFlow>> {
        Ok(None)
    }

    fn cursor_moved(
        &mut self,
        event_loop: &LoopTarget,
        device_id: We::DeviceId,
        position: Wd::PhysicalPosition<f64>,
    ) -> Result<Option<Wl::ControlFlow>> {
        Ok(None)
    }

    fn modifiers_changed(
        &mut self,
        event_loop: &LoopTarget,
        state: We::ModifiersState,
    ) -> Result<Option<Wl::ControlFlow>> {
        Ok(None)
    }

    fn received_character(
        &mut self,
        event_loop: &LoopTarget,
        ch: char,
    ) -> Result<Option<Wl::ControlFlow>> {
        Ok(None)
    }

    fn keyboard_input(
        &mut self,
        event_loop: &LoopTarget,
        device_id: We::DeviceId,
        input: We::KeyboardInput,
        is_synthetic: bool,
    ) -> Result<Option<Wl::ControlFlow>> {
        self.common_keyboard_input(event_loop, device_id, input, is_synthetic)
    }

    fn resized(
        &mut self,
        event_loop: &LoopTarget,
        size: Wd::PhysicalSize<u32>,
    ) -> Result<Option<Wl::ControlFlow>> {
        Ok(None)
    }

    fn close_requested(&mut self, event_loop: &LoopTarget) -> Result<Option<Wl::ControlFlow>> {
        Ok(Some(Wl::ControlFlow::ExitWithCode(0)))
    }

    fn destroyed(self, event_loop: &LoopTarget) -> Result<Option<Wl::ControlFlow>> {
        Ok(None)
    }

    fn common_keyboard_input(
        &mut self,
        event_loop: &LoopTarget,
        device_id: We::DeviceId,
        input: We::KeyboardInput,
        is_synthetic: bool,
    ) -> Result<Option<Wl::ControlFlow>> {
        if let Some(v) = input.virtual_keycode {
            match v {
                We::VirtualKeyCode::Escape | We::VirtualKeyCode::Q => {
                    Ok(Some(Wl::ControlFlow::ExitWithCode(0)))
                }
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
