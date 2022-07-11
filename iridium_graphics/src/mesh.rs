use iridium_maths::VecN;

/// Stores data about a vertex.
pub struct Vertex {
    /// The position.
    pub position: VecN<3>,
    /// The UV coordinates.
    pub uv: VecN<2>,
}

impl Vertex {
    /// Creates a new vertex.
    #[must_use]
    pub const fn new(position: VecN<3>, uv: VecN<2>) -> Self {
        Self { position, uv }
    }

    /// Convert to bytes to be sent to the shader.
    #[must_use]
    pub fn as_bytes(&self) -> [u8; 20] {
        let mut bytes = [0u8; 20];

        self.position
            .as_bytes::<12>()
            .into_iter()
            .chain(self.uv.as_bytes::<8>().into_iter())
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
