use hashbrown::HashMap;
use iridium_assets::Assets;

use crate::{ComponentBox, ComponentTrait};

/// A field in a `StoredComponent`.
pub struct StoredComponentField {
    /// The data of the field.
    pub string: String,
    /// Whether the field is a string.
    ///
    /// If true, it's stored in json as `"x"` instead of `x`.
    pub is_string: bool,
}

impl StoredComponentField {
    /// Creates a new `StoredComponentField`.
    #[must_use]
    pub const fn new(string: String, is_string: bool) -> Self {
        Self { string, is_string }
    }

    /// Creates a `StoredComponentField` from a json5 value.
    #[must_use]
    pub fn from_json5(value: &str) -> Self {
        if value.starts_with('"') && value.ends_with('"') {
            Self::new(value[1..value.len() - 1].to_string(), true)
        } else {
            Self::new(value.to_string(), false)
        }
    }
}

/// A component as it is stored.
///
/// This is created before storage, and returned from storage.
pub struct StoredComponent {
    /// The type name of the component.
    pub type_name: String,
    /// The fields of the component.
    ///
    /// This may not be a 1:1 mapping to the fields of the component,
    /// as some fields are only important at runtime.
    pub fields: HashMap<String, StoredComponentField>,
}

impl StoredComponent {
    /// Gets a value from the fields.
    ///
    /// This is moved from the fields.
    pub fn get(&mut self, key: &str) -> Option<String> {
        Some(self.fields.remove(key)?.string)
    }
}

/// A component that can be stored.
pub trait ComponentStorage {
    /// Try to create a component from a stored component.
    ///
    /// This returns an Option as the user may have corrupted save data
    /// so it may be invalid.
    fn from_stored(stored: StoredComponent, assets: &Assets) -> Option<Self>
    where
        Self: Sized;

    /// Try to create a component from a stored component.
    ///
    /// Returns a `Component` instead of `Self`.
    #[must_use]
    fn from_stored_component(stored: StoredComponent, assets: &Assets) -> Option<ComponentBox>
    where
        Self: Sized + ComponentTrait,
    {
        Self::from_stored(stored, assets).map(|t| ComponentBox::new(t))
    }

    /// Create a stored component from a component.
    fn to_stored(&self) -> StoredComponent;
}
