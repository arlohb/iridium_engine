use hashbrown::HashMap;
use iridium_assets::Assets;

use crate::{World, Component, Entities, Name, storage::StoredComponentField};

use super::StoredComponent;

/// Used in an intermediary step when reading a world.
/// 
/// This created after the file has been read, and all the fields extracted,
/// but before the component's fields are turned to components.
/// 
/// This is not guaranteed to make valid components.
pub type StoredEntities = Vec<(u128, Vec<StoredComponent>)>;

/// Used in an intermediary step when reading a world.
/// 
/// While this looks close to how entities are stored in world,
/// the world is more complicated, hence storing them this way at first.
/// 
/// This is guaranteed to be valid.
pub type ParsedEntities = Vec<(u128, Name, Vec<Component>)>;

/// The location an error occurred at.
#[derive(Debug)]
pub enum ErrorLocation {
    /// The error is at this line.
    Line(u32),
    /// The error is at the component with this id.
    Component(u128),
}

/// An error that occurred when reading a saved world.
#[derive(Debug)]
pub enum ReadError {
    /// The file could not be opened.
    FileNotFound(String),
    /// The syntax was invalid at the given line.
    SyntaxError(ErrorLocation),
    /// The component name was not recognised.
    UnknownComponent(ErrorLocation, String),
    /// The component had an invalid field.
    InvalidField(ErrorLocation),
    /// The entity is missing the `Name` component.
    MissingName(u128),
}

fn read_file(file: &str) -> Result<String, ReadError> {
    std::fs::read_to_string(file).map_err(|_| ReadError::FileNotFound(file.to_string()))
}

fn extract_entities(src: &str) -> Result<StoredEntities, ReadError> {
    enum State {
        Start,
        None,
        Entities,
        Entity,
        Component,
    }

    let mut current_component: Option<StoredComponent> = None;
    let mut current_entity: Option<(u128, Vec<StoredComponent>)> = None;

    let mut stored_entities = StoredEntities::new();
    let mut state = State::Start;

    for (line_number, line) in src.lines().enumerate() {
        match &mut state {
            State::Start => {
                if line.starts_with('{') {
                    state = State::None;
                }
            }
            State::None => {
                if line.trim_start().starts_with("entities: {") {
                    state = State::Entities;
                }
            }
            State::Entities => {
                if let Some(id_str) = line.trim_start().strip_prefix('"').and_then(|line| line.strip_suffix("\": {")) {
                    let id = id_str.parse::<u128>().unwrap();
                    current_entity = Some((id, Vec::new()));
                    state = State::Entity;
                } else if line.trim() != "}" || line.trim() != "}," {
                    state = State::None;
                } else {
                    return Err(ReadError::SyntaxError(ErrorLocation::Line(line_number as u32)));
                }
            }
            State::Entity => {
                if let Some(component_name) = line.strip_suffix(": {") {
                    let component_name = component_name.trim_start();
                    current_component = Some(StoredComponent {
                        type_name: component_name.to_string(),
                        fields: HashMap::new(),
                    });
                    state = State::Component;
                } else if line.trim() == "}" || line.trim() == "}," {
                    stored_entities.push(current_entity.take().unwrap());
                    state = State::Entities;
                } else {
                    return Err(ReadError::SyntaxError(ErrorLocation::Line(line_number as u32)));
                }
            }
            State::Component => {
                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim().to_string();
                    let value = StoredComponentField::from_json5(value.trim().trim_end_matches(','));

                    current_component.as_mut().unwrap().fields.insert(key, value);
                } else if line.trim() == "}" || line.trim() == "}," {
                    current_entity.as_mut().unwrap().1.push(current_component.take().unwrap());
                    state = State::Entity;
                } else {
                    return Err(ReadError::SyntaxError(ErrorLocation::Line(line_number as u32)));
                }
            }
        }
    }

    Ok(stored_entities)
}

fn parse_components(entities: &Entities, assets: &Assets, stored_entities: StoredEntities) -> Result<ParsedEntities, ReadError> {
    let mut parsed_entities = Vec::with_capacity(stored_entities.len());

    for (id, stored_components) in stored_entities {
        let mut parsed_components = Vec::with_capacity(stored_components.len());
        let mut name = None;

        for stored in stored_components {
            let from_stored_fn = entities.component_info_from_name(&stored.type_name)
                .ok_or_else(|| ReadError::UnknownComponent(ErrorLocation::Component(id), stored.type_name.clone()))?
                .from_stored;

            // ONLY TEMPORARY.
            // Lots of things needs to change in `Renderable2D` to make it savable,
            // until then we just ignore any fails.

            // let component = from_stored_fn(stored, assets)
            //     .ok_or(ReadError::InvalidField(ErrorLocation::Component(id)))?;

            let component = match from_stored_fn(stored, assets) {
                Some(component) => component,
                None => continue,
            };
            
            if component.type_name() == "Name" {
                name = Some(component);
            } else {
                parsed_components.push(component);
            }
        }

        parsed_entities.push((
            id,
            name.ok_or(ReadError::MissingName(id))?.take::<Name>(),
            parsed_components,
        ));
    }

    Ok(parsed_entities)
}

fn write_components_to_world(parsed_entities: ParsedEntities, world: &mut World) {
    world.entities.clear();

    for (id, name, components) in parsed_entities {
        world.entities.new_entity_with_id(id, &name.name, components);
    }
}

/// A simple wrapper around `StorageReader` to load a world from a file.
/// 
/// In the future this may return a world instead of modifying an existing one,
/// but right now it needs the existing systems as they aren't serialized.
pub fn load_world_from_file(file: &str, world: &mut World, assets: &Assets) -> Result<(), ReadError> {
    let src = read_file(file)?;

    let stored_entities = extract_entities(&src)?;

    let parsed_entities = parse_components(&world.entities, assets, stored_entities)?;

    write_components_to_world(parsed_entities, world);

    Ok(())
}
