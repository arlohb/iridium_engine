#![warn(
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    clippy::nursery,
    future_incompatible
)]
#![allow(clippy::module_name_repetitions)]

//! # Iridium Assets
//!
//! This provides functionality to load and manage assets.

mod assets;
pub use assets::*;

mod asset;
pub use asset::*;
