//! This crate is for stuff that is shared between the editor a project.

mod project_settings;
pub use project_settings::*;

mod key_code;
pub use key_code::*;

mod mouse_button;
pub use mouse_button::*;

mod input;
pub use input::*;

mod log;
pub use log::*;

mod project;
pub use project::*;
