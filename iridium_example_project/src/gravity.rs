use iridium_assets::Assets;
use iridium_ecs::{storage::*, systems::System, *};
use iridium_ecs_macros::{system_helper, ComponentStorage, ComponentTrait, InspectorUi};
use iridium_map_utils::fast_map;

use crate::Velocity;

#[derive(ComponentTrait, InspectorUi, ComponentStorage, Default)]
pub struct Weight;

#[derive(ComponentTrait, InspectorUi)]
pub struct GravityState {
    pub acceleration: f32,
}

impl Default for GravityState {
    fn default() -> Self {
        Self { acceleration: 9.81 }
    }
}

impl ComponentStorage for GravityState {
    fn from_stored(mut stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        Some(GravityState {
            acceleration: stored.get("acceleration")?.parse().ok()?,
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "GravityState".to_string(),
            fields: fast_map! {
                "acceleration" => StoredComponentField::new(self.acceleration.to_string(), false),
            },
        }
    }
}

pub struct GravitySystem;

impl GravitySystem {
    fn system(state: &mut GravityState, entities: &Entities, _assets: &Assets, delta_time: f64) {
        for (velocity, _) in entities.query::<(&mut Velocity, &Weight)>() {
            *velocity.velocity.y_mut() -= state.acceleration * delta_time as f32;
        }
    }
}

#[system_helper(GravityState)]
impl System for GravitySystem {}
