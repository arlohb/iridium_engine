use std::sync::Arc;

use iridium_graphics::*;

pub struct Assets {
    pub shaders: Vec<Arc<Shader>>,
    pub materials: Vec<Arc<Material>>,
    pub meshes: Vec<Arc<Mesh>>,
}
