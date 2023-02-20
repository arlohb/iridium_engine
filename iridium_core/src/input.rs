use iridium_ecs_macros::{Component, ComponentStorage, InspectorUi};
use iridium_maths::VecN;
use std::collections::HashMap;
use winit::event::{MouseButton, VirtualKeyCode};

/// The pressed state of a button.
///
/// This could be a mouse button or a key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonState {
    /// The button is not pressed.
    Up,
    /// The button has just been pressed this frame.
    Pressed,
    /// The button is currently being held down.
    Held,
    /// The button has just been released this frame.
    Released,
}

/// Stores the state of the input.
///
/// Despite its name, this is not the state of a system,
/// as there'd be nothing for the system to do.
///
/// It is stored under the name "`SystemState`" in the world.
#[derive(Component, InspectorUi, ComponentStorage)]
pub struct InputState {
    /// The current mouse position.
    #[temporary(VecN::zero())]
    pub mouse_position: VecN<2>,
    /// A map of the mouse buttons.
    #[hidden]
    #[temporary(HashMap::new())]
    pub mouse_buttons: HashMap<MouseButton, ButtonState>,
    /// A map of all the buttons.
    ///
    /// This isn't public because if a key is not in the map, it is assumed to be up,
    /// which access functions deal with.
    #[hidden]
    #[temporary(HashMap::new())]
    inputs: HashMap<VirtualKeyCode, ButtonState>,
}

impl InputState {
    /// Egui key to winit key.
    #[must_use]
    pub const fn egui_to_winit_key(key: egui::Key) -> winit::event::VirtualKeyCode {
        use egui::Key;

        match key {
            Key::ArrowDown => VirtualKeyCode::Down,
            Key::ArrowLeft => VirtualKeyCode::Left,
            Key::ArrowRight => VirtualKeyCode::Right,
            Key::ArrowUp => VirtualKeyCode::Up,
            Key::Escape => VirtualKeyCode::Escape,
            Key::Tab => VirtualKeyCode::Tab,
            Key::Backspace => VirtualKeyCode::Back,
            Key::Enter => VirtualKeyCode::Return,
            Key::Space => VirtualKeyCode::Space,
            Key::Insert => VirtualKeyCode::Insert,
            Key::Delete => VirtualKeyCode::Delete,
            Key::Home => VirtualKeyCode::Home,
            Key::End => VirtualKeyCode::End,
            Key::PageUp => VirtualKeyCode::PageUp,
            Key::PageDown => VirtualKeyCode::PageDown,
            Key::Num0 => VirtualKeyCode::Numpad0,
            Key::Num1 => VirtualKeyCode::Numpad1,
            Key::Num2 => VirtualKeyCode::Numpad2,
            Key::Num3 => VirtualKeyCode::Numpad3,
            Key::Num4 => VirtualKeyCode::Numpad4,
            Key::Num5 => VirtualKeyCode::Numpad5,
            Key::Num6 => VirtualKeyCode::Numpad6,
            Key::Num7 => VirtualKeyCode::Numpad7,
            Key::Num8 => VirtualKeyCode::Numpad8,
            Key::Num9 => VirtualKeyCode::Numpad9,
            Key::A => VirtualKeyCode::A,
            Key::B => VirtualKeyCode::B,
            Key::C => VirtualKeyCode::C,
            Key::D => VirtualKeyCode::D,
            Key::E => VirtualKeyCode::E,
            Key::F => VirtualKeyCode::F,
            Key::G => VirtualKeyCode::G,
            Key::H => VirtualKeyCode::H,
            Key::I => VirtualKeyCode::I,
            Key::J => VirtualKeyCode::J,
            Key::K => VirtualKeyCode::K,
            Key::L => VirtualKeyCode::L,
            Key::M => VirtualKeyCode::M,
            Key::N => VirtualKeyCode::N,
            Key::O => VirtualKeyCode::O,
            Key::P => VirtualKeyCode::P,
            Key::Q => VirtualKeyCode::Q,
            Key::R => VirtualKeyCode::R,
            Key::S => VirtualKeyCode::S,
            Key::T => VirtualKeyCode::T,
            Key::U => VirtualKeyCode::U,
            Key::V => VirtualKeyCode::V,
            Key::W => VirtualKeyCode::W,
            Key::X => VirtualKeyCode::X,
            Key::Y => VirtualKeyCode::Y,
            Key::Z => VirtualKeyCode::Z,
            Key::F1 => VirtualKeyCode::F1,
            Key::F2 => VirtualKeyCode::F2,
            Key::F3 => VirtualKeyCode::F3,
            Key::F4 => VirtualKeyCode::F4,
            Key::F5 => VirtualKeyCode::F5,
            Key::F6 => VirtualKeyCode::F6,
            Key::F7 => VirtualKeyCode::F7,
            Key::F8 => VirtualKeyCode::F8,
            Key::F9 => VirtualKeyCode::F9,
            Key::F10 => VirtualKeyCode::F10,
            Key::F11 => VirtualKeyCode::F11,
            Key::F12 => VirtualKeyCode::F12,
            Key::F13 => VirtualKeyCode::F13,
            Key::F14 => VirtualKeyCode::F14,
            Key::F15 => VirtualKeyCode::F15,
            Key::F16 => VirtualKeyCode::F16,
            Key::F17 => VirtualKeyCode::F17,
            Key::F18 => VirtualKeyCode::F18,
            Key::F19 => VirtualKeyCode::F19,
            Key::F20 => VirtualKeyCode::F20,
        }
    }

    /// Maps egui mouse button to winit mouse button.
    #[must_use]
    pub const fn egui_to_winit_mouse_button(
        button: egui::PointerButton,
    ) -> winit::event::MouseButton {
        use egui::PointerButton;

        match button {
            PointerButton::Primary => MouseButton::Left,
            PointerButton::Secondary => MouseButton::Right,
            PointerButton::Middle => MouseButton::Middle,
            PointerButton::Extra1 => MouseButton::Other(3),
            PointerButton::Extra2 => MouseButton::Other(4),
        }
    }

    /// Process the inputs from the last frame.
    ///
    /// The frame specific inputs are moved on.
    pub fn process_old_inputs(&mut self) {
        for state in self.inputs.values_mut() {
            match state {
                ButtonState::Pressed => *state = ButtonState::Held,
                ButtonState::Released => *state = ButtonState::Up,
                _ => {}
            }
        }
    }

    /// Should only be used by integrations,
    /// unless you want to fake input.
    pub fn key_pressed(&mut self, key: VirtualKeyCode) {
        self.inputs.insert(key, ButtonState::Pressed);
    }

    /// Should only be used by integrations,
    /// unless you want to fake input.
    pub fn key_released(&mut self, key: VirtualKeyCode) {
        self.inputs.insert(key, ButtonState::Released);
    }

    /// Should only be used by integrations,
    /// unless you want to fake input.
    pub fn mouse_button_pressed(&mut self, button: MouseButton) {
        self.mouse_buttons.insert(button, ButtonState::Pressed);
    }

    /// Should only be used by integrations,
    /// unless you want to fake input.
    pub fn mouse_button_released(&mut self, button: MouseButton) {
        self.mouse_buttons.insert(button, ButtonState::Released);
    }

    /// Get a key state.
    #[must_use]
    pub fn key(&self, key: VirtualKeyCode) -> ButtonState {
        self.inputs.get(&key).copied().unwrap_or(ButtonState::Up)
    }

    /// Get a mouse button state.
    #[must_use]
    pub fn mouse_button(&self, button: MouseButton) -> ButtonState {
        self.mouse_buttons
            .get(&button)
            .copied()
            .unwrap_or(ButtonState::Up)
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            mouse_position: VecN::zero(),
            mouse_buttons: HashMap::new(),
            inputs: HashMap::new(),
        }
    }
}
