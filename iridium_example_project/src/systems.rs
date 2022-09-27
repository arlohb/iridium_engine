use iridium_assets::Assets;
use iridium_ecs::{storage::*, systems::System, *};
use iridium_ecs_macros::{ComponentTrait, InspectorUi};
use iridium_map_utils::fast_map;

use crate::components::Custom;

#[derive(ComponentTrait, InspectorUi)]
pub struct CustomState {
    pub test: f64,
}

impl ComponentStorage for CustomState {
    fn from_stored(mut stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        Some(CustomState {
            test: stored.get("test")?.parse().ok()?,
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "CustomState".to_string(),
            fields: fast_map! {
                "test" => StoredComponentField::NonString(self.test.to_string()),
            },
        }
    }
}

pub struct CustomSystem;

impl System for CustomSystem {
    fn name(&self) -> &'static str {
        "CustomSystem"
    }

    fn state_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<CustomState>()
    }

    fn default_state(&self) -> Component {
        Component::new(CustomState { test: 0. })
    }

    fn system(&self, state: &Component, entities: &Entities, _assets: &Assets, _delta_time: f64) {
        let _state = state.get::<CustomState>();

        for (custom_component,) in entities.query::<(&mut Custom,)>() {
            println!("Custom component value: {}", custom_component.test);
        }
    }
}
