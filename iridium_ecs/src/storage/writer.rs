use crate::{ComponentBox, Entities, World};
use std::fmt::Write;

/// Manages the process of saving data to a file.
pub struct StorageWriter {
    /// The path of the file to write to.
    pub dst_path: String,
    buffer: String,
}

impl StorageWriter {
    /// Create a new writer.
    #[must_use]
    pub const fn new(dst_path: String) -> Self {
        Self {
            dst_path,
            buffer: String::new(),
        }
    }

    /// Write a component to the file.
    fn write_component(&mut self, component: &ComponentBox) {
        let stored = component.get_trait().to_stored();

        writeln!(&mut self.buffer, "            {}: {{", stored.type_name).unwrap();

        for (key, value) in stored.fields {
            write!(&mut self.buffer, "                {key}: ").unwrap();

            if value.is_string {
                write!(&mut self.buffer, "\"{}\"", value.string).unwrap();
            } else {
                self.buffer.push_str(&value.string);
            }

            self.buffer.push_str(",\n");
        }

        self.buffer.push_str("            },\n");
    }

    /// Write an entity to the file.
    fn write_entity(&mut self, entities: &Entities, id: u128) {
        writeln!(&mut self.buffer, "        \"{id}\": {{").unwrap();

        for component in entities
            .get_entity_components(id)
            .expect("Entity did not exist")
        {
            self.write_component(component);
        }

        self.buffer.push_str("        },\n");
    }

    /// Write the entities to the file.
    pub fn write_entities(&mut self, entities: &Entities) {
        self.buffer.push_str("    entities: {\n");

        for id in entities.entity_ids() {
            self.write_entity(entities, id);
        }

        self.buffer.push_str("    },\n");
    }

    /// Begins the json string.
    pub fn begin(&mut self) {
        self.buffer.push_str("{\n");
    }

    /// Ends the json string.
    pub fn end(&mut self) {
        self.buffer.push_str("}\n");
    }

    /// Writes the final buffer to the file.
    pub fn write(self) {
        std::fs::write(self.dst_path, self.buffer.as_bytes()).expect("Failed to write file");
    }

    /// Save the world to the file.
    ///
    /// Should be called in-between `begin` and `end`.
    pub fn save_world(&mut self, world: &World) {
        self.write_entities(&world.entities);
    }
}

/// A simple wrapper around `StorageWriter` to save the world to a file.
pub fn save_world_to_file(world: &World, file: &str) {
    let mut writer = StorageWriter::new(file.to_string());

    writer.begin();
    writer.save_world(world);
    writer.end();

    writer.write();
}
