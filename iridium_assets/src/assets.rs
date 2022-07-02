use std::sync::Arc;
use hashbrown::HashMap;

use iridium_graphics::*;

/// The asset manager to store all assets such as textures, shaders, etc.
pub struct Assets {
    /// The map of all textures.
    pub textures: HashMap<String, Arc<Texture>>,
    /// The map of all shaders.
    pub shaders: HashMap<String, Arc<Shader>>,
    /// The map of all materials.
    pub materials: HashMap<String, Arc<Material>>,
    /// The map of all meshes.
    pub meshes: HashMap<String, Arc<Mesh>>,
}
