use iridium_ecs::Transform;
use iridium_ecs_macros::{Component, ComponentStorage, InspectorUi};

/// A rectangle that stores the min and max points.
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

/// A collider that can detect collisions with itself.
///
/// It uses AABB collision, so can only model rectangles.
#[derive(Component, InspectorUi, ComponentStorage)]
pub struct AABBCollider {
    /// The center x coord.
    pub x: f64,
    /// The x size.
    pub x_size: f64,
    /// The center y coord.
    pub y: f64,
    /// The y size.
    pub y_size: f64,
    /// A tag to identify the collider.
    pub tag: String,
}

impl Default for AABBCollider {
    fn default() -> Self {
        Self {
            x: 0.,
            x_size: 1.,
            y: 0.,
            y_size: 1.,
            tag: String::new(),
        }
    }
}

impl AABBCollider {
    /// Check whether two rects are colliding.
    ///
    /// This should be called after offsets are calculated.
    #[must_use]
    pub fn are_rects_colliding(a: &Rect, b: &Rect) -> bool {
        let x_collide = a.max_x >= b.min_x && b.max_x >= a.min_x;
        let y_collide = a.max_y >= b.min_y && b.max_y >= a.min_y;

        x_collide && y_collide
    }

    /// Check whether this and another collider are colliding.
    ///
    /// This takes in the other collider's transform and calculates the offsets.
    #[must_use]
    pub fn is_colliding(&self, self_t: &Transform, other: &Self, other_t: &Transform) -> bool {
        let self_rect = Rect {
            min_x: (self.x - self.x_size / 2.) as f32 + self_t.position.x(),
            max_x: (self.x + self.x_size / 2.) as f32 + self_t.position.x(),
            min_y: (self.y - self.y_size / 2.) as f32 + self_t.position.y(),
            max_y: (self.y + self.y_size / 2.) as f32 + self_t.position.y(),
        };

        let other_rect = Rect {
            min_x: (other.x - other.x_size / 2.) as f32 + other_t.position.x(),
            max_x: (other.x + other.x_size / 2.) as f32 + other_t.position.x(),
            min_y: (other.y - other.y_size / 2.) as f32 + other_t.position.y(),
            max_y: (other.y + other.y_size / 2.) as f32 + other_t.position.y(),
        };

        Self::are_rects_colliding(&self_rect, &other_rect)
    }
}
