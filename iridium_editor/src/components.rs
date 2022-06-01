use hashbrown::HashMap;
use iridium_ecs::{ComponentType, fast_map, create_component_types};

pub fn component_types() -> HashMap<String, ComponentType> {
    create_component_types! {
        "Name" => fast_map! {
            "name" => "String"
        },
        "Transform" => fast_map! {
            "position" => "iridium_maths::Vec3",
            "scale" => "iridium_maths::Vec3"
        },
        "Velocity" => fast_map! {
            "velocity" => "iridium_maths::Vec3"
        }
    }
}
