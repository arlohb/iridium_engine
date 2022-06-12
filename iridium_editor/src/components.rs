use hashbrown::HashMap;
use iridium_ecs::{ComponentType, create_component_types};

pub fn component_types() -> HashMap<String, ComponentType> {
    create_component_types! {
        struct Name {
            name: String,
        },
        struct Transform {
            position: iridium_maths::Vec3,
            scale: iridium_maths::Vec3,
            rotation: f32,
        },
        struct Velocity {
            velocity: iridium_maths::Vec3,
        },
    }
}
