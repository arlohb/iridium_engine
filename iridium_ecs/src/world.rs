use crate::*;
use iridium_assets::Assets;
use storage::*;
use systems::*;

/// The world of the game.
///
/// The top level struct for the ECS.
///
/// Stores all entities, components and systems.
pub struct World {
    /// The entities.
    ///
    /// This is separate to world to allow sharing references to entities without systems.
    pub entities: Entities,
    /// The systems.
    systems: Systems,
}

impl World {
    /// Creates a new world with the given entities and systems.
    ///
    /// System state is automatically added here.
    pub fn new(mut entities: Entities, systems: Systems) -> World {
        // Add the system state to the world.
        entities.new_entity("SystemState", systems.default_component_states());

        World { entities, systems }
    }

    /// Runs the world's systems.
    pub fn run_systems(&mut self, delta_time: f64) {
        self.systems.run_systems(&self.entities, delta_time);
    }

    /// Saves the world's state to the given file.
    pub fn save(&self, file: &str) {
        save_world_to_file(self, file);
    }

    /// Loads the world's state from the given file.
    pub fn load(&mut self, file: &str, assets: &Assets) -> Result<(), ReadError> {
        load_world_from_file(file, self, assets)
    }
}
