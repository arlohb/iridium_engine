use std::any::Any;

use iridium_assets::Assets;

use crate::{storage::ComponentStorage, ui::InspectorUi};

/// A trait implemented by components.
pub trait Component: 'static + Send + Sync + Any + ComponentStorage + InspectorUi {
    /// The name of the component type.
    ///
    /// Called on the type.
    fn type_name() -> &'static str
    where
        Self: Sized;

    /// The name of the component type.
    ///
    /// Called on an instance of the type.
    fn dyn_type_name(&self) -> &'static str;

    /// This updates all the asset fields of the component if needed.
    /// It returns the numbers of assets that were reloaded.
    ///
    /// # Errors
    ///
    /// If any of the new assets aren't found.
    fn update_assets(&mut self, assets: &Assets) -> Result<i32, String>;
}
