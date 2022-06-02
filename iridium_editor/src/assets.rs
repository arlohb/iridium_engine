use std::sync::Arc;

use iridium_graphics::*;

pub struct Assets {
    pub textures: Vec<Arc<Texture>>,
    pub shaders: Vec<Arc<Shader>>,
    pub materials: Vec<Arc<Material>>,
    pub meshes: Vec<Arc<Mesh>>,
}
