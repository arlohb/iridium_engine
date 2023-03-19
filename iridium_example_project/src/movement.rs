use iridium_core::{InputState, KeyCode};
use iridium_ecs::Transform;
use iridium_ecs_macros::{
    system_helper, Component, ComponentStorage, HasStableTypeId, InspectorUi,
};

/// Can be moved up and down by keyboard inputs.
#[derive(Component, InspectorUi, ComponentStorage, HasStableTypeId)]
pub struct Movement {
    /// The key to move up.
    #[id(0)]
    pub up: KeyCode,
    /// The key to move down.
    #[id(1)]
    pub down: KeyCode,
    /// The speed to move up and down.
    #[drag_speed(0.001)]
    pub speed: f32,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            up: KeyCode::ArrowUp,
            down: KeyCode::ArrowDown,
            speed: 0.001,
        }
    }
}

/// Moves entities with the `Movement` component.
pub struct MovementSystem;

impl MovementSystem {
    fn system(
        _: (),
        entities: &iridium_ecs::Entities,
        (_, transform, movement): (u128, &mut Transform, &Movement),
        _assets: &iridium_assets::Assets,
        delta_time: f64,
    ) -> Result<(), String> {
        let input_state = entities.get::<InputState>();

        if input_state.key(&movement.up).down() {
            *transform.position.y_mut() += movement.speed * delta_time as f32;
        }
        if input_state.key(&movement.down).down() {
            *transform.position.y_mut() -= movement.speed * delta_time as f32;
        }

        Ok(())
    }
}

#[system_helper((), par_iter, &mut Transform, &Movement)]
impl System for MovementSystem {}
