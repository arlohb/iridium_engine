use iridium_assets::Assets;
use iridium_core::{ButtonState, InputState, VirtualKeyCode};
use iridium_ecs_macros::{system_helper, Component, ComponentStorage, InspectorUi};

use crate::Velocity;

/// An entity that can fly when the player presses the space bar.
#[derive(Component, InspectorUi, ComponentStorage)]
pub struct Flight {
    /// The force applied upwards on flight.
    #[drag_speed(0.0001)]
    pub force: f32,
}

impl Default for Flight {
    fn default() -> Self {
        Self { force: 0.01 }
    }
}

/// The system that applies flight.
pub struct FlightSystem;

impl FlightSystem {
    fn system(
        _state: (),
        entities: &iridium_ecs::Entities,
        (_, velocity, flight): (u128, &mut Velocity, &Flight),
        _assets: &Assets,
        _delta_time: f64,
    ) -> Result<(), String> {
        let input_state = entities.get::<InputState>();

        if input_state.key(VirtualKeyCode::Space) == ButtonState::Pressed {
            *velocity.velocity.y_mut() = flight.force;
        }

        Ok(())
    }
}

#[system_helper((), par_iter, &mut Velocity, &Flight)]
impl System for FlightSystem {}
