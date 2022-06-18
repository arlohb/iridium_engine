use iridium_ecs_macros::ComponentTrait;

use crate::{ComponentFieldUi, ComponentFactory, Component};

use crate as iridium_ecs;

#[derive(ComponentTrait)]
pub struct Name {
    pub name: String,
}

#[derive(ComponentTrait)]
pub struct Transform {
    #[drag_speed(0.05)]
    pub position: iridium_maths::Vec3,
    #[drag_speed(0.05)]
    pub scale: iridium_maths::Vec3,
    #[drag_speed(0.05)]
    pub rotation: f32,
}

impl ComponentFactory for Transform {
    fn create() -> Component {
        Component::new(Self {
            position: iridium_maths::Vec3::new(0.0, 0.0, 0.0),
            scale: iridium_maths::Vec3::new(1.0, 1.0, 1.0),
            rotation: 0.0,
        })
    }
}

#[derive(ComponentTrait)]
pub struct Velocity {
    #[drag_speed(0.0001)]
    pub velocity: iridium_maths::Vec3,
}

impl ComponentFactory for Velocity {
    fn create() -> Component {
        Component::new(Self {
            velocity: iridium_maths::Vec3::new(0.0, 0.0, 0.0),
        })
    }
}
