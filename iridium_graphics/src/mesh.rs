use iridium_maths::Vec3;

pub struct Vertex {
    pub position: Vec3,
    // There aren't many uses for Vec2,
    // so for now we'll just use [f32; 2]
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn new(position: Vec3, uv: [f32; 2]) -> Self {
        Self {
            position,
            uv,
        }
    }

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

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}