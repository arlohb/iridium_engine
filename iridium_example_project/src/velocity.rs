use iridium_assets::Assets;
use iridium_ecs::{
    storage::{ComponentStorage, StoredComponent, StoredComponentField},
    systems::System,
    Component, Entities, Transform,
};
use iridium_ecs_macros::{system_helper, ComponentStorage, ComponentTrait, InspectorUi};
use iridium_map_utils::fast_map;

/// The velocity of an entity.
#[derive(ComponentTrait, InspectorUi)]
pub struct Velocity {
    #[drag_speed(0.0001)]
    /// The velocity.
    pub velocity: iridium_maths::VecN<3>,
}

impl ComponentStorage for Velocity {
    fn from_stored(mut stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        Some(Self {
            velocity: stored.get("velocity")?.parse().ok()?,
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "Velocity".to_string(),
            fields: fast_map! {
                "velocity" => StoredComponentField::new(self.velocity.to_string(), false),
            },
        }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            velocity: iridium_maths::VecN::new([0.0, 0.0, 0.0]),
        }
    }
}

#[derive(ComponentTrait, InspectorUi, ComponentStorage, Default)]
pub struct VelocityState;

pub struct VelocitySystem;

impl VelocitySystem {
    fn system(_state: &mut VelocityState, entities: &Entities, _assets: &Assets, delta_time: f64) {
        for (transform, velocity) in entities.query::<(&mut Transform, &mut Velocity)>() {
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
