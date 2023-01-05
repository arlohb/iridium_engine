use iridium_assets::Assets;
use iridium_ecs::{
    storage::{ComponentStorage, StoredComponent, StoredComponentField},
    Entities, Transform,
};
use iridium_ecs_macros::{system_helper, ComponentTrait, InspectorUi};
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

/// Applies velocity to entities with the `Velocity` component.
pub struct VelocitySystem;

impl VelocitySystem {
    fn system(
        _state: (),
        _entities: &Entities,
        (transform, velocity): (&mut Transform, &mut Velocity),
        _assets: &Assets,
        delta_time: f64,
    ) {
        let position = &mut transform.position;
        let velocity = &mut velocity.velocity;
        *position += *velocity * delta_time as f32;
    }
}

#[system_helper((), par_iter, &mut Transform, &mut Velocity)]
impl System for VelocitySystem {}
