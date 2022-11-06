use iridium_assets::Assets;
use iridium_ecs_macros::{ComponentTrait, InspectorUi};
use iridium_map_utils::fast_map;

use super::{Component, ComponentDefault};
use crate::storage::{ComponentStorage, StoredComponent, StoredComponentField};

/// The name of an entity.
///
/// Added by default to all entities on creation.
#[derive(ComponentTrait, InspectorUi)]
pub struct Name {
    /// The name of the entity.
    pub name: String,
}

impl ComponentStorage for Name {
    fn from_stored(mut stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        Some(Self {
            name: stored.get("name")?,
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "Name".to_string(),
            fields: fast_map! {
                "name" => StoredComponentField::new(self.name.clone(), true),
            },
        }
    }
}

/// The position, scale and rotation of an entity.
#[derive(ComponentTrait, InspectorUi)]
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

impl ComponentStorage for Transform {
    fn from_stored(mut stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        Some(Self {
            position: stored.get("position")?.parse().ok()?,
            scale: stored.get("scale")?.parse().ok()?,
            rotation: stored.get("rotation")?.parse().ok()?,
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "Transform".to_string(),
            fields: fast_map! {
                "position" => StoredComponentField::new(self.position.to_string(), false),
                "scale" => StoredComponentField::new(self.scale.to_string(), false),
                "rotation" => StoredComponentField::new(self.rotation.to_string(), false),
            },
        }
    }
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
#[derive(ComponentTrait, InspectorUi)]
pub struct Velocity {
    #[drag_speed(0.0001)]
    /// The velocity.
    pub velocity: iridium_maths::VecN<3>,
}

impl ComponentStorage for Velocity {
    fn from_stored(mut stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        Some(Self {
            velocity: stored.get("velocity")?.parse().ok()?,
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "Velocity".to_string(),
            fields: fast_map! {
                "velocity" => StoredComponentField::new(self.velocity.to_string(), false),
            },
        }
    }
}

impl ComponentDefault for Velocity {
    fn create() -> Component {
        Component::new(Self {
            velocity: iridium_maths::VecN::new([0.0, 0.0, 0.0]),
        })
    }
}
