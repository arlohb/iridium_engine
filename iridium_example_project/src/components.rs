use iridium_ecs_macros::ComponentTrait;
use iridium_ecs::{ComponentFieldUi, storage::*};
use iridium_map_utils::fast_map;

#[derive(ComponentTrait)]
pub struct Custom {
    pub test: f64,
}

impl ComponentStorage for Custom {
    fn from_stored(mut stored: StoredComponent) -> Option<Self> {
        Some(Custom {
            test: stored.get("test")?.parse().ok()?,
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "Custom".to_string(),
            fields: fast_map! {
                "test" => StoredComponentField::NonString(self.test.to_string()),
            },
        }
    }
}
