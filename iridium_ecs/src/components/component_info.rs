use iridium_assets::Assets;

use crate::storage::StoredComponent;

use super::{Component, ComponentDefault, ComponentTrait};

/// Information about a component type when it is registered.
///
/// Right now this is just the type name, in the future this may include field types.
pub struct ComponentInfo {
    /// The name of the component type.
    pub type_name: &'static str,
    /// Creates a component from within the UI.
    ///
    /// Not all components implement this.
    pub default: Option<fn() -> Component>,
    /// Tries to create a component from a stored component.
    pub from_stored: fn(StoredComponent, &Assets) -> Option<Component>,
}

impl ComponentInfo {
    /// Creates a new component info from a component type.
    #[must_use]
    pub fn new<T: ComponentTrait>() -> Self {
        Self {
            type_name: T::type_name(),
            default: None,
            from_stored: T::from_stored_component,
        }
    }

    /// Creates a new component info from a component type.
    ///
    /// Also adds the default fn.
    #[must_use]
    pub fn new_with_default<T: ComponentTrait + ComponentDefault>() -> Self {
        Self {
            type_name: T::type_name(),
            default: Some(T::create),
            from_stored: T::from_stored_component,
        }
    }
}
