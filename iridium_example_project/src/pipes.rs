use iridium_assets::Assets;
use iridium_core::LogState;
use iridium_ecs::{EntityCommand, Transform};
use iridium_ecs_macros::{system_helper, Component, ComponentStorage, InspectorUi};
use iridium_graphics::Renderable2D;
use iridium_maths::VecN;
use rand::Rng;

use crate::Velocity;

/// Just a marker component for pipes.
#[derive(Component, InspectorUi, ComponentStorage, Default)]
pub struct Pipe;

/// The system that removes pipes from the world.
pub struct PipeRemovalSystem;

impl PipeRemovalSystem {
    fn system(
        _state: (),
        entities: &iridium_ecs::Entities,
        (id, transform, _): (u128, &Transform, &Pipe),
        _assets: &Assets,
        _delta_time: f64,
    ) -> Result<(), String> {
        if transform.position.x() <= -1. {
            entities.send_cmd(EntityCommand::DeleteEntity(id));
        }

        Ok(())
    }
}

#[system_helper((), par_iter, &Transform, &Pipe)]
impl System for PipeRemovalSystem {}

/// The state for the `PipeSystem`.
#[derive(Component, InspectorUi, ComponentStorage)]
pub struct PipeState {
    /// The min time between pipes in secs.
    pub min_time_gap: f64,
    /// The max time between pipes in secs.
    pub max_time_gap: f64,
    /// The padding between the top / bottom of the screen and a pipe gap.
    #[drag_speed(0.001)]
    pub edge_padding: f64,
    /// The min gap height between the pipes.
    #[drag_speed(0.001)]
    pub gap_min: f64,
    /// The max gap height between the pipes.
    #[drag_speed(0.001)]
    pub gap_max: f64,
    /// The speed of the pipes.
    #[drag_speed(0.001)]
    pub pipe_speed: f64,
    /// The current time until the next pipe in secs.
    #[temporary(0f64)]
    pub next_pipe_in: f64,
}

impl Default for PipeState {
    fn default() -> Self {
        Self {
            min_time_gap: 1.,
            max_time_gap: 2.,
            edge_padding: 0.2,
            gap_min: 0.25,
            gap_max: 0.4,
            pipe_speed: 0.001,
            next_pipe_in: 0.,
        }
    }
}

/// The system that places pipes in the world.
pub struct PipeSystem;

impl PipeSystem {
    fn create_pipe_pair(
        state: &PipeState,
        entities: &iridium_ecs::Entities,
        assets: &Assets,
    ) -> Result<(), String> {
        let gap_height = rand::thread_rng().gen_range(state.gap_min..state.gap_max);
        let gap_center = rand::thread_rng().gen_range(
            (-1. + state.edge_padding + gap_height)..(1. - state.edge_padding - gap_height),
        ) as f32;
        let gap_height = gap_height as f32;

        Self::create_pipe(
            state,
            entities,
            assets,
            Transform {
                position: VecN::new([1., 1., 0.]),
                scale: VecN::new([0.6, 1. - gap_center - (gap_height / 2.), 1.]),
                rotation: std::f32::consts::PI,
            },
        )?;

        Self::create_pipe(
            state,
            entities,
            assets,
            Transform {
                position: VecN::new([1., -1., 0.]),
                scale: VecN::new([0.6, 2. - (1. - gap_center + (gap_height / 2.)), 1.]),
                rotation: 0.,
            },
        )?;

        Ok(())
    }

    fn create_pipe(
        state: &PipeState,
        entities: &iridium_ecs::Entities,
        assets: &Assets,
        transform: Transform,
    ) -> Result<(), String> {
        entities.send_cmd(EntityCommand::NewEntity(
            None,
            "Pipe".to_owned(),
            vec![
                transform.into(),
                Renderable2D::new(assets.get("quad_offset")?, assets.get("wine_mat")?).into(),
                Velocity {
                    velocity: VecN::new([-state.pipe_speed as f32, 0., 0.]),
                }
                .into(),
                Pipe.into(),
            ],
        ));

        Ok(())
    }

    fn system(
        state: &mut PipeState,
        entities: &iridium_ecs::Entities,
        assets: &Assets,
        delta_time: f64,
    ) -> Result<(), String> {
        state.next_pipe_in -= delta_time / 1000.;

        if state.next_pipe_in <= 0. {
            state.next_pipe_in =
                rand::thread_rng().gen_range(state.min_time_gap..state.max_time_gap);

            Self::create_pipe_pair(state, entities, assets)?;

            entities.get::<LogState>().info("Placed a new pipe.");
        }

        Ok(())
    }
}

#[system_helper(PipeState, once)]
impl System for PipeSystem {}
