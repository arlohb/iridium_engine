use hashbrown::HashMap;
use iridium_ecs::{ComponentType, fast_map, create_component_types};

pub fn component_types() -> HashMap<String, ComponentType> {
    create_component_types! {
        "Name" => fast_map! {
            "name" => "String"
        },
        "Position" => fast_map! {
            "x" => "f64",
            "y" => "f64",
            "z" => "f64"
        },
        "Velocity" => fast_map! {
            "x" => "f64",
            "y" => "f64",
            "z" => "f64"
        }
    }
}
