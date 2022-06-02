use hashbrown::HashMap;
use iridium_ecs::{ComponentType, create_component_types};
use iridium_map_utils::fast_map;

pub fn component_types() -> HashMap<String, ComponentType> {
    create_component_types! {
        "Name" => fast_map! {
            "name" => "String",
        },
        "Transform" => fast_map! {
            "position" => "iridium_maths::Vec3",
            "scale" => "iridium_maths::Vec3",
            "rotation" => "f32",
        },
        "Velocity" => fast_map! {
            "velocity" => "iridium_maths::Vec3",
        },
    }
}
