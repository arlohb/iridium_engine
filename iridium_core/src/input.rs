use iridium_ecs_macros::{Component, ComponentStorage, HasStableTypeId, InspectorUi};
use iridium_maths::VecN;
use std::collections::HashMap;

use crate::{KeyCode, MouseButton};

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
#[derive(Component, InspectorUi, ComponentStorage, HasStableTypeId)]
pub struct InputState {
    /// The current mouse position in logical pixels.
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
    inputs: HashMap<KeyCode, ButtonState>,
}

impl InputState {
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
    pub fn key_pressed(&mut self, key: KeyCode) {
        self.inputs.insert(key, ButtonState::Pressed);
    }

    /// Should only be used by integrations,
    /// unless you want to fake input.
    pub fn key_released(&mut self, key: KeyCode) {
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
    pub fn key(&self, key: &KeyCode) -> ButtonState {
        self.inputs.get(key).copied().unwrap_or(ButtonState::Up)
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
