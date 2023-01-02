//! This crate is for stuff that is shared between the editor a project.

mod project_settings;
pub use project_settings::*;

mod input;
pub use input::*;

// Re-export this.
pub use winit::event::VirtualKeyCode;
