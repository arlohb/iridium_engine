use iridium_assets::Assets;
use iridium_core::{ButtonState, InputState, VirtualKeyCode};
use iridium_ecs::storage::{ComponentStorage, StoredComponent, StoredComponentField};
use iridium_ecs_macros::{system_helper, ComponentTrait, InspectorUi};
use iridium_map_utils::fast_map;

use crate::Velocity;

/// An entity that can fly when the player presses the space bar.
#[derive(ComponentTrait, InspectorUi)]
pub struct Flight {
    /// The force applied upwards on flight.
    pub force: f32,
}

impl ComponentStorage for Flight {
    fn from_stored(mut stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        Some(Self {
            force: stored.get("force")?.parse().ok()?,
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "Flight".to_string(),
            fields: fast_map! {
                "force" => StoredComponentField::new(self.force.to_string(), false),
            },
        }
    }
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
    ) {
        let input_state = entities.get::<InputState>();

        if input_state.key(VirtualKeyCode::Space) == ButtonState::Pressed {
            *velocity.velocity.y_mut() = flight.force;
        }
    }
}

#[system_helper((), par_iter, &mut Velocity, &Flight)]
impl System for FlightSystem {}
