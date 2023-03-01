use iridium_assets::Assets;
use iridium_collision::Rect;
use iridium_ecs::{query, Transform};
use iridium_ecs_macros::{
    system_helper, Component, ComponentStorage, HasStableTypeId, InspectorUi,
};
use iridium_graphics::Renderable2D;

use crate::Velocity;

/// Will make the ball bounce off it.
#[derive(Component, ComponentStorage, InspectorUi, Default, HasStableTypeId)]
pub struct Wall {
    /// Whether the wall it horizonal or vertical.
    vertical: bool,
}

/// Detects collisions between pipes and the bird.
///
/// The `Flight` component is used to find the bird.
pub struct CollisionSystem;

impl CollisionSystem {
    #[allow(clippy::similar_names)]
    fn system(
        _state: (),
        entities: &iridium_ecs::Entities,
        (_, wall, wall_transform, wall_r2d): (u128, &Wall, &Transform, &Renderable2D),
        _assets: &Assets,
        _delta_time: f64,
    ) -> Result<(), String> {
        let wall_rect = Rect::bounding_from_vertices(
            &wall_r2d
                .mesh
                .vertices
                .iter()
                .map(|vertex| vertex.position)
                .collect::<Vec<_>>(),
        )
        .apply_transform(wall_transform);

        for (_, velocity, ball_transform, ball_r2d) in
            query!(entities, [mut Velocity; Transform, Renderable2D])
        {
            let ball_rect = Rect::bounding_from_vertices(
                &ball_r2d
                    .mesh
                    .vertices
                    .iter()
                    .map(|vertex| vertex.position)
                    .collect::<Vec<_>>(),
            )
            .apply_transform(ball_transform);

            if ball_rect.is_colliding(&wall_rect) {
                if wall.vertical {
                    *velocity.velocity.x_mut() *= -1.;
                } else {
                    *velocity.velocity.y_mut() *= -1.;
                }
            }
        }

        Ok(())
    }
}

#[system_helper((), par_iter, &Wall, &Transform, &Renderable2D)]
impl System for CollisionSystem {}
