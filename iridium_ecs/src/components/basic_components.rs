use iridium_ecs_macros::{Component, ComponentStorage, InspectorUi};

/// The name of an entity.
///
/// Added by default to all entities on creation.
#[derive(Component, InspectorUi, ComponentStorage)]
pub struct Name {
    /// The name of the entity.
    #[string]
    pub name: String,
}

/// The position, scale and rotation of an entity.
#[derive(Component, InspectorUi, ComponentStorage)]
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

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: iridium_maths::VecN::new([0.0, 0.0, 0.0]),
            scale: iridium_maths::VecN::new([1.0, 1.0, 1.0]),
            rotation: 0.0,
        }
    }
}
