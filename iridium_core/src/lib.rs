//! This crate is for stuff that is shared between the editor a project.

mod project_settings;
pub use project_settings::*;

mod input;
pub use input::*;

mod log;
pub use log::*;

mod project;
pub use project::*;

// Re-export this.
pub use winit::event::VirtualKeyCode;
