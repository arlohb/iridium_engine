use iridium_assets::Assets;
use iridium_ecs_macros::{ComponentTrait, InspectorUi};
use iridium_map_utils::fast_map;

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

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: iridium_maths::VecN::new([0.0, 0.0, 0.0]),
            scale: iridium_maths::VecN::new([1.0, 1.0, 1.0]),
            rotation: 0.0,
        }
    }
}
