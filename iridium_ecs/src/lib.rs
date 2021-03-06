#![warn(
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    clippy::nursery,
    future_incompatible
)]
#![allow(clippy::module_name_repetitions)]

//! # Iridium Ecs
//!
//! This is the entity-component-system (ECS) portion of Iridium.
//!
//! This started out as independent of the editor UI,
//!
//! but it massively simplified the code the integrate UI into components.

mod components;
pub use components::*;
mod entities;
pub use entities::*;
mod world;
pub use world::*;

/// System management.
pub mod systems;

/// Storage management.
pub mod storage;
