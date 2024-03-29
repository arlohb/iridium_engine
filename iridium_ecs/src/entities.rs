#![allow(clippy::mut_from_ref)]

use std::sync::mpsc;

use iridium_assets::Assets;

use crate::ComponentDefault;
use iridium_reflect::{HasStableTypeId, StableTypeId};

use super::{Component, ComponentBox, ComponentInfo, Name, Transform};
use std::collections::HashMap;

/// A command created in a system, to be ran
/// by `Entities` at the end of each stage.
pub enum EntityCommand {
    /// Delete the entity with the given ID.
    DeleteEntity(u128),
    /// Create an entity, maybe with the given id.
    NewEntity(Option<u128>, String, Vec<ComponentBox>),
}

/// Stores all the entities in the scene.
pub struct Entities {
    /// entity_id => components
    entities: HashMap<u128, Vec<StableTypeId>>,
    /// component_type => entity_id => component
    components: HashMap<StableTypeId, HashMap<u128, ComponentBox>>,
    /// Stores info about components.
    component_info: HashMap<StableTypeId, ComponentInfo>,

    /// The mpsc receiver for the entity commands.
    cmd_queue: mpsc::Receiver<EntityCommand>,
    /// The mpsc sender for the entity commands.
    cmd_sender: mpsc::Sender<EntityCommand>,
}

// The only blocker for this being automatically implemented
// is cmd_sender not being Sync.
//
// I think this is safe, as I only ever clone it on other threads,
// and don't send with it.
unsafe impl Sync for Entities {}

impl Default for Entities {
    fn default() -> Self {
        let (cmd_sender, cmd_queue) = mpsc::channel();

        // Create Entities.
        let mut entities = Self {
            entities: HashMap::new(),
            components: HashMap::new(),
            component_info: HashMap::new(),
            cmd_queue,
            cmd_sender,
        };

        // Register the default components.
        entities.register_component::<Name>();
        entities.register_component_with_default::<Transform>();

        entities
    }
}

impl Entities {
    /// Deletes all entities and components from a scene.
    pub fn clear(&mut self) {
        self.entities.clear();
        self.components.clear();
    }

    /// Updates all the assets on all the components.
    ///
    /// # Errors
    ///
    /// If a new asset id is not found.
    pub fn update_assets(&self, assets: &Assets) -> Result<i32, String> {
        puffin::profile_function!();

        Ok(self
            .components
            .values()
            .map(|component_map| {
                component_map
                    .values()
                    .map(|component| component.get_trait_mut().update_assets(assets))
                    // This gets the first err if found
                    .collect::<Result<Vec<i32>, String>>()
            })
            // This gets the first err if found
            // And then early returns if err
            .collect::<Result<Vec<Vec<i32>>, String>>()?
            .into_iter()
            .flatten()
            .sum::<i32>())
    }

    /// Send an `EntityCommand`.
    pub fn send_cmd(&self, cmd: EntityCommand) {
        self.cmd_sender
            // I clone it because `mpsc::Sender` is not Sync, only Send
            .clone()
            .send(cmd)
            .unwrap_or_else(|_| {
                unreachable!("Reciever can't have been dropped as it's in the same struct")
            });
    }

    /// Process the commands in the command queue.
    pub fn process_commands(&mut self) {
        while let Ok(cmd) = self.cmd_queue.try_recv() {
            match cmd {
                EntityCommand::DeleteEntity(entity_id) => {
                    self.delete_entity(entity_id);
                }
                EntityCommand::NewEntity(id, name, components) => {
                    self.new_entity(id, &name, components);
                }
            }
        }
    }

    /// Delete an entity and all its components.
    ///
    /// Returns true if the entity was deleted,
    /// false if it didn't exist.
    pub fn delete_entity(&mut self, entity_id: u128) -> bool {
        // Remove the entity from the entities list,
        // and get the components it had.
        let Some(component_types) = self.entities.remove(&entity_id) else {
            return false;
        };

        // For each component the entity had,
        // remove it from the components list.
        for component_type in component_types {
            self.components
                .get_mut(&component_type)
                .expect("Invalid component type in entity while deleting")
                .remove(&entity_id);
        }

        true
    }

    /// Gets `ComponentInfo` from the component type.
    #[must_use]
    pub fn component_info<T: Component>(&self) -> Option<&ComponentInfo> {
        self.component_info.get(&T::stable_type_id())
    }

    /// Gets `ComponentInfo` from the component type name.
    #[must_use]
    pub fn component_info_from_name(&self, name: &str) -> Option<&ComponentInfo> {
        self.component_info
            .iter()
            .find(|(_, info)| info.type_name == name)
            .map(|(_, info)| info)
    }

    /// Gets `ComponentInfo` from the component type id.
    #[must_use]
    pub fn component_info_from_type_id(&self, type_id: StableTypeId) -> Option<&ComponentInfo> {
        self.component_info.get(&type_id)
    }

    /// Gets the number of entities with a given component.
    #[must_use]
    pub fn entity_count<T: Component>(&self) -> usize {
        self.components
            .get(&T::stable_type_id())
            .map_or(0, HashMap::len)
    }

    /// Gets an entity id from its name.
    #[must_use]
    pub fn entity_id_from_name(&self, name: &str) -> Option<u128> {
        self.query_by_type_id([Name::stable_type_id()])
            .find(|(_, [name_component])| name_component.get::<Name>().name == name)
            .map(|(id, _)| id)
    }

    /// Gets a vec of all entity ids.
    #[must_use]
    pub fn entity_ids(&self) -> Vec<u128> {
        self.entities.keys().copied().collect()
    }

    /// Registers a component type.
    ///
    /// This stores info about the component.
    pub fn register_component<T: Component>(&mut self) {
        let component_info = ComponentInfo::new::<T>();
        self.component_info
            .insert(T::stable_type_id(), component_info);
    }

    /// Registers a component type with a default implementation.
    ///
    /// Called instead of `register_component`
    pub fn register_component_with_default<T: Component + ComponentDefault>(&mut self) {
        let component_info = ComponentInfo::new_with_default::<T>();
        self.component_info
            .insert(T::stable_type_id(), component_info);
    }

    /// Get a vec of component names and their factories.
    #[allow(clippy::type_complexity)]
    #[must_use]
    pub fn component_defaults(
        &self,
    ) -> Vec<(&'static str, fn(&Assets) -> Result<ComponentBox, String>)> {
        self.component_info
            .iter()
            .filter_map(|(_, info)| Some((info.type_name, info.default?)))
            .collect::<Vec<_>>()
    }

    /// Add components to an entity.
    pub fn add_components(&mut self, entity_id: u128, components: Vec<ComponentBox>) {
        // Get the vec of components the entities has.
        // If it doesn't exist, add it.
        let entity = self.entities.entry(entity_id).or_default();

        // For each component to be added.
        for component in components {
            // If the component is already added, continue.
            if entity.contains(&component.stable_type_id()) {
                continue;
            }

            // Add to entities.
            entity.push(component.stable_type_id());

            // Get the hashmap of type_id => component.
            self.components
                .entry(component.stable_type_id())
                // If it doesn't exist, add it.
                .or_insert_with(HashMap::new)
                // Add this component to the hashmap.
                .insert(entity_id, component);
        }
    }

    /// Creates a new entity with the given components and id.
    ///
    /// If id is none, a new id is generated.
    /// Returns the id of the new entity.
    ///
    /// Automatically adds the Name component with the given name.
    pub fn new_entity(
        &mut self,
        id: Option<u128>,
        name: &str,
        components: Vec<ComponentBox>,
    ) -> u128 {
        // Generate a new entity id if none is given.
        let id = id.unwrap_or_else(|| uuid::Uuid::new_v4().as_u128());

        // Add it to entities.
        self.entities.insert(id, vec![]);

        // Add the name component.
        self.add_components(
            id,
            vec![Name {
                name: name.to_owned(),
            }
            .into()],
        );

        if !components.is_empty() {
            // Add the other components.
            self.add_components(id, components);
        }

        id
    }

    /// Get all the component types an entity has.
    ///
    /// Returns None if the entity doesn't exist.
    #[must_use]
    pub fn get_entity_component_types(&self, entity_id: u128) -> Option<Vec<StableTypeId>> {
        // Do a simple look up in the entities map.
        self.entities.get(&entity_id).cloned()
    }

    /// Get all the components of a given entity.
    ///
    /// Returns None if the entity doesn't exist.
    #[must_use]
    pub fn get_entity_components(&self, entity_id: u128) -> Option<Vec<&ComponentBox>> {
        // Get the component types of the entity.
        let component_types = self.get_entity_component_types(entity_id)?;

        // For each component the entity has.
        Some(
            component_types
                .into_iter()
                .map(|component_type| {
                    // Get the map of entities => components.
                    let entities_to_components = self
                        .components
                        .get(&component_type)
                        .expect("Component type not registered.");
                    // Return the component.
                    entities_to_components
                        .get(&entity_id)
                        .expect("Component not found in entity.")
                    // Collect into a HashMap
                })
                .collect::<Vec<_>>(),
        )
    }

    /// Get an iterator over components of given types, in the form (entity id, \[comp1, comp2, comp3\]).
    ///
    /// # Panics
    #[must_use]
    pub fn query_by_type_id<const N: usize>(
        &self,
        component_types: [StableTypeId; N],
    ) -> std::vec::IntoIter<(u128, [&ComponentBox; N])> {
        puffin::profile_function!();

        // If only one component type is given,
        // we can skip all the hard work.
        if N == 1 {
            // Get the map of entities => components.
            let entities_to_components = self
                .components
                .get(&component_types[0])
                .expect("Component type not registered.");
            // Return the iterator.
            return entities_to_components
                .iter()
                .map(|(&entity_id, component)| {
                    // Return the entity id and the component.
                    (entity_id, [component; N])
                })
                .collect::<Vec<_>>()
                .into_iter();
        }

        // Find all the entities that have each component.
        let mut entities_with_each_component: Vec<Vec<u128>> = {
            puffin::profile_scope!("entities_with_each_component");

            component_types
                .into_iter()
                // Get all the entities that have each component.
                .map(|component_type| {
                    self.components
                        // Get entities => components for this component type.
                        .get(&component_type)
                        // Do this if previous is Some.
                        .map_or_else(Vec::new, |map| {
                            map
                                // Only get the entity id.
                                .keys()
                                // From &u128 -> u128
                                .copied()
                                // Into a vector.
                                .collect::<Vec<u128>>()
                        })
                })
                .collect()
        };

        // Find the intersection of the previous.
        // This is the set of entities that have all the components.
        let entities_with_all_components = {
            puffin::profile_scope!("entities_with_all_components");

            let mut ids = vec![];

            for id in entities_with_each_component
                .pop()
                .expect("This shouldn't happen, as length was checked before")
            {
                let mut in_all = true;

                for other_ids in &mut entities_with_each_component {
                    if let Some(index) = other_ids.iter().position(|x| *x == id) {
                        other_ids.remove(index);
                    } else {
                        in_all = false;
                        break;
                    };
                }

                if in_all {
                    ids.push(id);
                }
            }

            ids
        };

        puffin::profile_scope!("components_from_ids");

        // Create the final return value for each entity previously found.
        entities_with_all_components
            .into_iter()
            // For each entity id.
            .map(|id| {
                // Get the given components for the entity.
                let components_vec = component_types
                    .into_iter()
                    // For each component type.
                    .map(|component_type| {
                        // Get the map of entities => components.
                        let entities_to_components = &self.components[&component_type];
                        // Return the component.
                        &entities_to_components[&id]
                    })
                    // Into a vector.
                    .collect::<Vec<_>>();

                // This converts a vector to a sized array.
                let components_array: [&ComponentBox; N] =
                    components_vec.try_into().unwrap_or_else(|_| panic!());

                // Join with the id in a tuple.
                (id, components_array)
            })
            // Into a vector to evaluate everything.
            .collect::<Vec<(u128, [_; N])>>()
            // Into an iterator for ease of use in a system.
            .into_iter()
    }

    /// Get a single component of a given type.
    ///
    /// This gets the first component of the given type,
    ///
    /// but should only be used when you're sure there is only one.
    #[must_use]
    pub fn get<T: Component>(&self) -> &mut T {
        self.get_by_type_id(T::stable_type_id()).get_mut::<T>()
    }

    /// Get a single component with a given stable type id.
    ///
    /// This gets the first component of the given type,
    ///
    /// but should only be used when you're sure there is only one.
    #[must_use]
    pub fn get_by_type_id(&self, component_type: StableTypeId) -> &ComponentBox {
        self.components[&component_type]
            .values()
            .next()
            .expect("Component not found.")
    }
}

/// Queries the entities that have a set of components.
///
/// Used as `query(&Entities, [mut Component1, mut Component2 etc ; Component3, Component4 etc])`.
///
/// Returns an iterator of tuples of the form (id, Component1, Component2 etc).
///
/// # Examples
///
/// ```ignore
/// for (_id, transform, velocity)
/// in query!(&entities, [mut Transform; Velocity]) {
///    transform.position += velocity.velocity;
/// }
/// ```
#[macro_export]
macro_rules! query {
    ($entities:expr, [$(mut $mut_type:ty),* $(,)?; $($type:ty),* $(,)?]) => {
        {
            let type_ids = [
                $(
                    <$mut_type as iridium_reflect::HasStableTypeId>::stable_type_id(),
                )*
                $(
                    <$type as iridium_reflect::HasStableTypeId>::stable_type_id(),
                )*
            ];

            $entities.query_by_type_id(type_ids).map(|(id, components)| {
                let mut index = 0;
                (
                    id,
                    $(
                        {
                            #![allow(clippy::mixed_read_write_in_expression)]
                            index += 1;
                            components[index - 1].get_mut::<$mut_type>()
                        },
                    )*
                    $(
                        {
                            #![allow(clippy::mixed_read_write_in_expression)]
                            index += 1;
                            components[index - 1].get::<$type>()
                        },
                    )*
                )
            }).collect::<Vec<_>>().into_iter()
        }
    };
}
