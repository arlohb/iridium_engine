/// The type returns by `HasStableTypeId::STABLE_TYPE_ID`.
pub type StableTypeId = u64;

/// A type id that doesn't change between compiler invocations.
/// Calculated at compile time from the type name.
pub trait HasStableTypeId {
    /// Get the stable type id from the type.
    fn stable_type_id() -> StableTypeId
    where
        Self: Sized;

    /// Get the stable type id from an instance of self.
    fn dyn_stable_type_id(&self) -> StableTypeId;
}

impl HasStableTypeId for () {
    fn stable_type_id() -> StableTypeId {
        0
    }

    fn dyn_stable_type_id(&self) -> StableTypeId {
        0
    }
}
