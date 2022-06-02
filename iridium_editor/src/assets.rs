use std::sync::Arc;
use hashbrown::HashMap;

use iridium_graphics::*;

pub struct Assets {
    pub textures: HashMap<String, Arc<Texture>>,
    pub shaders: HashMap<String, Arc<Shader>>,
    pub materials: HashMap<String, Arc<Material>>,
    pub meshes: HashMap<String, Arc<Mesh>>,
}
