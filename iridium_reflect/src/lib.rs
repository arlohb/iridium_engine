//! # Iridium Reflect
//!
//! For now this just includes `StableTypeId`,
//! which is a type id that's consistent between compiler involcations.
//!
//! At some later point this might include more type reflection helpers.

mod stable_type_id;
pub use stable_type_id::*;
