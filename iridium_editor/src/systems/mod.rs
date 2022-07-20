mod frame_history;
pub use frame_history::*;

use iridium_assets::Assets;
use iridium_ecs::{
    query,
    storage::{ComponentStorage, StoredComponent, StoredComponentField},
    systems::System,
    Component, Entities, Transform, Velocity,
};
use iridium_ecs_macros::{system_helper, ComponentTrait, InspectorUi};
use iridium_map_utils::fast_map;

#[derive(ComponentTrait, InspectorUi)]
pub struct VelocityState {
    #[drag_speed(0.001)]
    pub rotation_speed: f32,
}

impl ComponentStorage for VelocityState {
    fn from_stored(mut stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        Some(Self {
            rotation_speed: stored.get("rotation_speed")?.parse().ok()?,
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "VelocityState".to_string(),
            fields: fast_map! {
                "rotation_speed" => StoredComponentField::NonString(self.rotation_speed.to_string()),
            },
        }
    }
}

impl Default for VelocityState {
    fn default() -> Self {
        Self {
            rotation_speed: 0.002,
        }
    }
}

pub struct VelocitySystem;

impl VelocitySystem {
    fn system(state: &mut VelocityState, entities: &Entities, delta_time: f64) {
        for (transform, velocity) in query!(entities, [mut Transform, mut Velocity;]) {
            transform.rotation += state.rotation_speed * delta_time as f32;

            let position = &mut transform.position;
            let velocity = &mut velocity.velocity;
            *position += *velocity * delta_time as f32;

            if position.x() < -1. {
                *position.x_mut() = -1.;
                *velocity.x_mut() = -velocity.x();
            }
            if position.x() > 1. {
                *position.x_mut() = 1.;
                *velocity.x_mut() = -velocity.x();
            }
            if position.y() < -1. {
                *position.y_mut() = -1.;
                *velocity.y_mut() = -velocity.y();
            }
            if position.y() > 1. {
                *position.y_mut() = 1.;
                *velocity.y_mut() = -velocity.y();
            }
        }
    }
}

#[system_helper(VelocityState)]
impl System for VelocitySystem {}

#[allow(dead_code)]
#[derive(ComponentTrait, InspectorUi)]
pub struct PositionLoggerState {}

impl ComponentStorage for PositionLoggerState {
    fn from_stored(_stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        Some(Self {})
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "PositionLoggerState".to_string(),
            fields: fast_map! {},
        }
    }
}

// This is a system to test other things,
// so is not always used.
#[allow(dead_code)]
pub struct PositionLoggerSystem;

impl System for PositionLoggerSystem {
    fn name(&self) -> &'static str {
        "PositionLoggerSystem"
    }

    fn state_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<PositionLoggerState>()
    }

    fn default_state(&self) -> Component {
        Component::new(PositionLoggerState {})
    }

    fn system(&self, state: &Component, entities: &Entities, _delta_time: f64) {
        let _state = state.get_mut::<PositionLoggerState>();

        for (transform,) in query!(entities, [; Transform]) {
            let position = transform.position;
            println!("{} {} {}", position.x(), position.y(), position.z());
        }
    }
}
