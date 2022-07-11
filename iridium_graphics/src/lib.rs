#![warn(
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    clippy::nursery,
    future_incompatible
)]
#![allow(clippy::module_name_repetitions)]

//! The graphics crate for iridium.
//!
//! Provides components and systems for rendering 2D graphics with WGPU.

mod components;
pub use components::*;

mod systems;
pub use systems::*;

mod shader;
pub use shader::*;

mod material;
pub use material::*;

mod mesh;
pub use mesh::*;

mod texture;
pub use texture::*;
