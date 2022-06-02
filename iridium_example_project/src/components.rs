use hashbrown::HashMap;
use iridium_ecs::*;
use iridium_map_utils::fast_map;

pub fn component_types() -> HashMap<String, ComponentType> {
    create_component_types! {
        "Custom" => fast_map! {
            "test" => "f64"
        }
    }
}
