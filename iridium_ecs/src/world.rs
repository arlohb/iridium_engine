use crate::{
    storage::{load_world_from_file, save_world_to_file, ReadError},
    systems::Systems,
    Entities,
};
use iridium_assets::Assets;

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
    #[must_use]
    pub fn new(mut entities: Entities, systems: Systems) -> Self {
        // Add the system state to the world.
        entities.new_entity("SystemState", systems.default_component_states());

        Self { entities, systems }
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
    ///
    /// # Errors
    ///
    /// Will return an error if the file cannot be read, or if the file is not a valid JSON5 file.
    pub fn load(&mut self, file: &str, assets: &Assets) -> Result<(), ReadError> {
        load_world_from_file(file, self, assets)
    }
}
