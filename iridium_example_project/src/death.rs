use iridium_assets::Assets;
use iridium_core::LogState;
use iridium_ecs::{Entities, EntityCommand, Name, Transform};
use iridium_ecs_macros::{system_helper, Component, ComponentStorage, InspectorUi};
use iridium_maths::VecN;

use crate::Velocity;

/// A thing that can die.
#[derive(Component, InspectorUi, ComponentStorage, Default)]
pub struct Death;

/// When an entity with a `Death` component hits the bottom of the screen,
/// it will die.
pub struct DeathSystem;

impl DeathSystem {
    fn system(
        _state: (),
        entities: &Entities,
        (id, transform, velocity, name, _death): (
            u128,
            &mut Transform,
            &mut Velocity,
            &Name,
            &Death,
        ),
        _assets: &Assets,
        _delta_time: f64,
    ) -> Result<(), String> {
        if transform.position.y() < -1. {
            entities
                .get::<LogState>()
                .info(format!("Entity {:?} died!", name.name));

            *transform.position.y_mut() = 0.;

            velocity.velocity = VecN::zero();

            entities.send_cmd(EntityCommand::DeleteEntity(id));
        }

        Ok(())
    }
}

#[system_helper((), par_iter, &mut Transform, &mut Velocity, &Name, &Death)]
impl System for DeathSystem {}
