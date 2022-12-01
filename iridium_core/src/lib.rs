//! This crate is for stuff that is shared between the editor a project.

/// Settings about the project.
pub struct ProjectSettings {
    /// The default scene to open when the editor opens or when the game starts.
    pub default_scene: String,
}
