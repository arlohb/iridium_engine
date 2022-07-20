use super::{Component, ComponentTrait};

/// A trait implemented by components that can be created from the UI without any inputs.
pub trait ComponentDefault: ComponentTrait + Sized {
    /// Creates a new component from the default values.
    ///
    /// This is returned as a Component, not Self.
    fn create() -> Component;
}
