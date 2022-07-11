#![warn(
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    clippy::nursery,
    future_incompatible
)]
#![allow(clippy::module_name_repetitions)]

//! A basic maths library for the Iridium game engine.

mod vecn;
pub use vecn::*;
