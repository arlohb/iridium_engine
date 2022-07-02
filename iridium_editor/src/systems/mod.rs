mod frame_history;
pub use frame_history::*;

use iridium_assets::Assets;
use iridium_ecs::{*, systems::System, storage::*};
use iridium_ecs_macros::ComponentTrait;
use iridium_map_utils::fast_map;

#[derive(ComponentTrait)]
pub struct VelocityState {
    #[drag_speed(0.001)]
    pub rotation_speed: f32,
}

impl ComponentStorage for VelocityState {
    fn from_stored(mut stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        Some(VelocityState {
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

pub struct VelocitySystem;

impl System for VelocitySystem {
    fn name(&self) -> &'static str { "VelocitySystem" }

    fn default_state(&self) -> Component {
        Component::new(VelocityState {
            rotation_speed: 0.002,
        })
    }

    fn system(&self, entities: &Entities, delta_time: f64) {
        for (transform, velocity)
        in query!(entities, [mut Transform, mut Velocity;]) {
            let state = entities.get::<VelocityState>();
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

#[allow(dead_code)]
#[derive(ComponentTrait)]
pub struct PositionLoggerState {}

impl ComponentStorage for PositionLoggerState {
    fn from_stored(_stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        Some(PositionLoggerState {})
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
    fn name(&self) -> &'static str { "PositionLoggerSystem" }

    fn default_state(&self) -> Component {
        Component::new(PositionLoggerState {})
    }

    fn system(&self, entities: &Entities, _delta_time: f64) {
        for (transform, )
        in query!(entities, [; Transform]) {
            let position = transform.position;
            println!("{} {} {}", position.x(), position.y(), position.z());
        }
    }
}
