use iridium_assets::Assets;
use iridium_ecs::storage::{ComponentStorage, StoredComponent, StoredComponentField};
use iridium_ecs_macros::{system_helper, ComponentTrait, InspectorUi};
use iridium_map_utils::fast_map;

/// The state for the `PipeSystem`.
#[derive(ComponentTrait, InspectorUi)]
pub struct PipeState {
    /// The min time between pipes in secs.
    pub min_time_gap: f64,
    /// The max time between pipes in secs.
    pub max_time_gap: f64,
    /// The current time until the next pipe in secs.
    pub next_pipe_in: f64,
}

impl Default for PipeState {
    fn default() -> Self {
        Self {
            min_time_gap: 1.,
            max_time_gap: 2.,
            next_pipe_in: 0.,
        }
    }
}

impl ComponentStorage for PipeState {
    fn from_stored(mut stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        Some(Self {
            min_time_gap: stored.get("min_time_gap")?.parse().ok()?,
            max_time_gap: stored.get("max_time_gap")?.parse().ok()?,
            next_pipe_in: 0.,
        })
    }

    fn to_stored(&self) -> iridium_ecs::storage::StoredComponent {
        StoredComponent {
            type_name: "PipeState".to_string(),
            fields: fast_map! {
                "min_time_gap".to_string() => StoredComponentField::new(self.min_time_gap.to_string(), false),
                "max_time_gap".to_string() => StoredComponentField::new(self.max_time_gap.to_string(), false),
            },
        }
    }
}

/// The system that places pipes in the world.
pub struct PipeSystem;

impl PipeSystem {
    fn system(
        state: &mut PipeState,
        _entities: &iridium_ecs::Entities,
        _assets: &Assets,
        delta_time: f64,
    ) {
        use rand::Rng;

        state.next_pipe_in -= delta_time / 1000.;

        if state.next_pipe_in <= 0. {
            state.next_pipe_in =
                rand::thread_rng().gen_range(state.min_time_gap..state.max_time_gap);

            // This is where a new pipe would be created,
            // but this can't be done yet.

            println!("Created a pipe");
        }
    }
}

#[system_helper(PipeState, once)]
impl System for PipeSystem {}
