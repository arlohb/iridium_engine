use iridium_assets::Assets;
use iridium_ecs::storage::*;
use iridium_ecs_macros::{ComponentTrait, InspectorUi};
use iridium_map_utils::fast_map;

#[derive(ComponentTrait, InspectorUi)]
pub struct Custom {
    pub test: f64,
}

impl ComponentStorage for Custom {
    fn from_stored(mut stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        Some(Custom {
            test: stored.get("test")?.parse().ok()?,
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "Custom".to_string(),
            fields: fast_map! {
                "test" => StoredComponentField::new(self.test.to_string(), false),
            },
        }
    }
}
