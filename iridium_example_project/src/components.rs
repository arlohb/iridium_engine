use hashbrown::HashMap;
use iridium_ecs::*;

pub fn component_types() -> HashMap<String, ComponentType> {
    create_component_types! {
        struct Custom {
            test: f64,
        },
    }
}
