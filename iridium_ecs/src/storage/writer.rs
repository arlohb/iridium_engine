use crate::{Component, Entities, World};

use super::StoredComponentField;

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
    fn write_component(&mut self, component: &Component) {
        let stored = component.get_trait().to_stored();

        self.buffer
            .push_str(&format!("            {}: {{\n", stored.type_name));

        for (key, value) in stored.fields {
            self.buffer.push_str(&format!("                {}: ", key));

            match value {
                StoredComponentField::String(string) => {
                    self.buffer.push_str(&format!("\"{string}\""));
                }
                StoredComponentField::NonString(string) => {
                    self.buffer.push_str(&string);
                }
            }

            self.buffer.push_str(",\n");
        }

        self.buffer.push_str("            },\n");
    }

    /// Write an entity to the file.
    fn write_entity(&mut self, entities: &Entities, id: u128) {
        self.buffer.push_str(&format!("        \"{id}\": {{\n"));

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
