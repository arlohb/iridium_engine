use hashbrown::HashMap;
use iridium_assets::Assets;

/// A field in a `StoredComponent`.
pub enum StoredComponentField {
    /// A string field.
    /// 
    /// Stored in json5 as `"x"` not `x`.
    String(String),
    /// A non-string field.
    /// 
    /// Stored in json5 as `x` not `"x"`.
    NonString(String),
}

impl StoredComponentField {
    /// Gets the string value.
    pub fn string(self) -> String {
        match self {
            StoredComponentField::String(s) => s,
            StoredComponentField::NonString(s) => s,
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
        Some(self.fields.remove(key)?.string())
    }
}

/// A component that can be stored.
pub trait ComponentStorage {
    /// Try to create a component from a stored component.
    /// 
    /// This returns an Option as the user may have corrupted save data
    /// so it may be invalid.
    fn from_stored(stored: StoredComponent, assets: &Assets) -> Option<Self>
        where Self: Sized;

    /// Create a stored component from a component.
    fn to_stored(&self) -> StoredComponent;
}
