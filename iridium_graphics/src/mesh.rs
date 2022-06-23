use iridium_maths::Vec3;

/// Stores data about a vertex.
pub struct Vertex {
    /// The position.
    pub position: Vec3,
    /// The UV coordinates.
    /// 
    /// There aren't many uses for Vec2,
    /// so for now we'll just use `[f32; 2]`.
    /// Perhaps I'll implement a Vec2 later.
    pub uv: [f32; 2],
}

impl Vertex {
    /// Creates a new vertex.
    pub fn new(position: Vec3, uv: [f32; 2]) -> Self {
        Self {
            position,
            uv,
        }
    }

    /// Convert to bytes to be sent to the shader.
    pub fn as_bytes(&self) -> [u8; 20] {
        let mut bytes = [0u8; 20];

        self.position.as_bytes::<12>().into_iter()
            .chain(self.uv[0].to_le_bytes().into_iter())
            .chain(self.uv[1].to_le_bytes().into_iter())
            .enumerate()
            .for_each(|(i, b)| bytes[i] = b);

        bytes
    }
}

/// Stores data about a mesh.
pub struct Mesh {
    /// The vertices.
    pub vertices: Vec<Vertex>,
    /// The indices.
    pub indices: Vec<u32>,
}