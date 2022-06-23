use iridium_ecs_macros::ComponentTrait;

use crate::{ComponentFieldUi, ComponentDefault, Component};

use crate as iridium_ecs;

/// The name of an entity.
/// 
/// Added by default to all entities on creation.
#[derive(ComponentTrait)]
pub struct Name {
    /// The name of the entity.
    pub name: String,
}

/// The position, scale and rotation of an entity.
#[derive(ComponentTrait)]
pub struct Transform {
    /// The position.
    #[drag_speed(0.05)]
    pub position: iridium_maths::VecN<3>,
    /// The scale.
    #[drag_speed(0.05)]
    pub scale: iridium_maths::VecN<3>,
    /// The rotation.
    /// 
    /// This is in radians.
    #[drag_speed(0.05)]
    pub rotation: f32,
}

impl ComponentDefault for Transform {
    fn create() -> Component {
        Component::new(Self {
            position: iridium_maths::VecN::new([0.0, 0.0, 0.0]),
            scale: iridium_maths::VecN::new([1.0, 1.0, 1.0]),
            rotation: 0.0,
        })
    }
}

/// The velocity of an entity.
#[derive(ComponentTrait)]
pub struct Velocity {
    #[drag_speed(0.0001)]
    /// The velocity.
    pub velocity: iridium_maths::VecN<3>,
}

impl ComponentDefault for Velocity {
    fn create() -> Component {
        Component::new(Self {
            velocity: iridium_maths::VecN::new([0.0, 0.0, 0.0]),
        })
    }
}
