use iridium_assets::Assets;

use crate::Component;

/// Creates a component filled with default values.
///
/// This trait is automatically implemented for
/// `T: Component + Default`,
/// and only needs to be manually implemented if
/// it needs to load default assets.
pub trait ComponentDefault {
    /// Creates a new component with default values.
    ///
    /// # Errors
    ///
    /// If default assets aren't found.
    fn default(assets: &Assets) -> Result<Self, String>
    where
        Self: Sized;
}

impl<T: Component + Default> ComponentDefault for T {
    fn default(_: &Assets) -> Result<Self, String> {
        Ok(T::default())
    }
}
