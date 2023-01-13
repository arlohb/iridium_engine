use std::any::Any;

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
}
