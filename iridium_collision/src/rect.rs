use iridium_ecs::Transform;
use iridium_maths::VecN;

/// A rectangle that stores the min and max points.
#[derive(Clone, Copy)]
pub struct Rect {
    /// The min x.
    pub min_x: f32,
    /// The max x.
    pub max_x: f32,
    /// The min y.
    pub min_y: f32,
    /// The max y.
    pub max_y: f32,
}

impl Rect {
    /// Create a rect that encompases the given vertices.
    #[must_use]
    pub fn bounding_from_vertices(vertices: &[VecN<3>]) -> Self {
        vertices.iter().fold(
            Self {
                min_x: std::f32::MAX,
                max_x: std::f32::MIN,
                min_y: std::f32::MAX,
                max_y: std::f32::MIN,
            },
            |mut rect, vertex| {
                rect.min_x = rect.min_x.min(vertex.x());
                rect.max_x = rect.max_x.max(vertex.x());
                rect.min_y = rect.min_y.min(vertex.y());
                rect.max_y = rect.max_y.max(vertex.y());

                rect
            },
        )
    }

    /// Check whether this and another rect are colliding.
    #[must_use]
    pub fn is_colliding(&self, other: &Self) -> bool {
        let x_collide = self.max_x >= other.min_x && other.max_x >= self.min_x;
        let y_collide = self.max_y >= other.min_y && other.max_y >= self.min_y;

        x_collide && y_collide
    }

    /// Rotate this rect.
    ///
    /// Rotation is in radians.
    #[must_use]
    pub fn rotate(self, rotation: f32) -> Self {
        let sin = rotation.sin();
        let cos = rotation.cos();

        let vertices = [
            VecN::new([self.min_x, self.min_y, 0.]),
            VecN::new([self.max_x, self.min_y, 0.]),
            VecN::new([self.max_x, self.max_y, 0.]),
            VecN::new([self.min_x, self.max_y, 0.]),
        ]
        .into_iter()
        .map(|vertex| {
            let x = vertex.x();
            let y = vertex.y();

            VecN::new([x.mul_add(cos, -y * sin), x.mul_add(sin, y * cos), 0.])
        })
        .collect::<Vec<_>>();

        Self::bounding_from_vertices(&vertices)
    }

    /// Scale this rect.
    #[must_use]
    pub fn scale(mut self, scale: VecN<3>) -> Self {
        self.min_x *= scale.x();
        self.max_x *= scale.x();
        self.min_y *= scale.y();
        self.max_y *= scale.y();

        self
    }

    /// Translate this rect.
    #[must_use]
    pub fn translate(mut self, translation: VecN<3>) -> Self {
        self.min_x += translation.x();
        self.max_x += translation.x();
        self.min_y += translation.y();
        self.max_y += translation.y();

        self
    }

    /// Apply a transform to this rect.
    #[must_use]
    pub fn apply_transform(mut self, t: &Transform) -> Self {
        // Rotate the rect.
        self = self.rotate(t.rotation);

        // Scale the rect.
        self = self.scale(t.scale);

        // Translate the rect.
        self = self.translate(t.position);

        self
    }
}
