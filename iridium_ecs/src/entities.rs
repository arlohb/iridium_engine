#![allow(clippy::mut_from_ref)]

use std::any::TypeId;

use crate::systems::SystemInputs;

use super::{
    Component, ComponentDefault, ComponentInfo, ComponentTrait, Name, Transform, Velocity,
};
use hashbrown::HashMap;

/// Stores all the entities in the scene.
pub struct Entities {
    /// entity_id => components
    entities: HashMap<u128, Vec<TypeId>>,
    /// component_type => entity_id => component
    components: HashMap<TypeId, HashMap<u128, Component>>,
    /// Stores info about components.
    component_info: HashMap<TypeId, ComponentInfo>,
}

impl Default for Entities {
    fn default() -> Self {
        // Create Entities.
        let mut entities = Self {
            entities: HashMap::new(),
            components: HashMap::new(),
            component_info: HashMap::new(),
        };

        // Register the default components.
        entities.register_component::<Name>();
        entities.register_component_with_default::<Transform>();
        entities.register_component_with_default::<Velocity>();

        entities
    }
}

impl Entities {
    /// Deletes all entities and components from a scene.
    pub fn clear(&mut self) {
        self.entities.clear();
        self.components.clear();
    }

    /// Gets `ComponentInfo` from the component type.
    #[must_use]
    pub fn component_info<T: ComponentTrait>(&self) -> Option<&ComponentInfo> {
        let type_id = TypeId::of::<T>();
        self.component_info.get(&type_id)
    }

    /// Gets `ComponentInfo` from the component type name.
    #[must_use]
    pub fn component_info_from_name(&self, name: &str) -> Option<&ComponentInfo> {
        self.component_info
            .iter()
            .find(|(_, info)| info.type_name == name)
            .map(|(_, info)| info)
    }

    /// Gets the number of entities with a given component.
    #[must_use]
    pub fn entity_count<T: ComponentTrait>(&self) -> usize {
        let type_id = TypeId::of::<T>();
        self.components.get(&type_id).map_or(0, HashMap::len)
    }

    /// Gets an entity id from its name.
    #[must_use]
    pub fn entity_id_from_name(&self, name: &str) -> Option<u128> {
        self.query_by_type_id_with_id([&std::any::TypeId::of::<Name>()])
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
    pub fn register_component<T: ComponentTrait>(&mut self) {
        let type_id = TypeId::of::<T>();
        let component_info = ComponentInfo::new::<T>();
        self.component_info.insert(type_id, component_info);
    }

    /// Registers a component type with a default implementation.
    ///
    /// Called instead of `register_component`
    pub fn register_component_with_default<T: ComponentTrait + ComponentDefault>(&mut self) {
        let type_id = TypeId::of::<T>();
        let component_info = ComponentInfo::new_with_default::<T>();
        self.component_info.insert(type_id, component_info);
    }

    /// Get a vec of component names and their factories.
    #[allow(clippy::type_complexity)]
    #[must_use]
    pub fn component_defaults(&self) -> Vec<(&'static str, fn() -> Component)> {
        self.component_info
            .iter()
            .filter_map(|(_, info)| Some((info.type_name, info.default?)))
            .collect::<Vec<_>>()
    }

    /// Add components to an entity.
    ///
    /// Takes a `Vec` instead of a `[Component; N]` if you don't know the number of components.
    pub fn add_components_dyn(&mut self, entity_id: u128, components: Vec<Component>) {
        // Get the vec of components the entities has.
        // If it doesn't exist, add it.
        let entity = self.entities.entry(entity_id).or_default();

        // For each component to be added.
        for component in components {
            // If the component is already added, continue.
            if entity.contains(&component.type_id()) {
                continue;
            }

            // Add to entities.
            entity.push(component.type_id());

            // Get the hashmap of type_id => component.
            self.components
                .entry(component.type_id())
                // If it doesn't exist, add it.
                .or_insert_with(HashMap::new)
                // Add this component to the hashmap.
                .insert(entity_id, component);
        }
    }

    /// Add components to an entity.
    pub fn add_components<const N: usize>(&mut self, entity_id: u128, components: [Component; N]) {
        self.add_components_dyn(entity_id, {
            let mut vec = Vec::with_capacity(N);

            for t in components {
                vec.push(t);
            }

            vec
        });
    }

    /// Create a new entity with the given components.
    ///
    /// Automatically adds the Name component with the given name.
    pub fn new_entity<const N: usize>(&mut self, name: &str, components: [Component; N]) -> u128 {
        // Generate a new entity id.
        let id = uuid::Uuid::new_v4().as_u128();

        self.new_entity_with_id(id, name, components);

        // Return the id.
        id
    }

    /// Creates a new entity with the given components and id.
    ///
    /// Automatically adds the Name component with the given name.
    pub fn new_entity_with_id<const N: usize>(
        &mut self,
        id: u128,
        name: &str,
        components: [Component; N],
    ) {
        // Add it to entities.
        self.entities.insert(id, vec![]);

        // Add the name component.
        self.add_components(
            id,
            [Component::new(Name {
                name: name.to_owned(),
            })],
        );

        if N > 0 {
            // Add the other components.
            self.add_components(id, components);
        }
    }

    /// Get all the component types an entity has.
    ///
    /// Returns None if the entity doesn't exist.
    #[must_use]
    pub fn get_entity_component_types(&self, entity_id: u128) -> Option<Vec<TypeId>> {
        // Do a simple look up in the entities map.
        self.entities.get(&entity_id).cloned()
    }

    /// Get all the components of a given entity.
    ///
    /// Returns None if the entity doesn't exist.
    #[must_use]
    pub fn get_entity_components(&self, entity_id: u128) -> Option<Vec<&Component>> {
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
    pub fn query_by_type_id_with_id<const N: usize>(
        &self,
        component_types: [&TypeId; N],
    ) -> std::vec::IntoIter<(u128, [&Component; N])> {
        puffin::profile_function!();

        // If only one component type is given,
        // we can skip all the hard work.
        if N == 1 {
            // Get the map of entities => components.
            let entities_to_components = self
                .components
                .get(component_types[0])
                .expect("Component type not registered.");
            // Return the iterator.
            entities_to_components
                .iter()
                .map(|(&entity_id, component)| {
                    // Return the entity id and the component.
                    (entity_id, [component; N])
                })
                .collect::<Vec<_>>()
                .into_iter()
        } else {
            // Find all the entities that have each component.
            let entities_with_each_component = {
                puffin::profile_scope!("entities_with_each_component");

                component_types
                    .into_iter()
                    // Get all the entities that have each component.
                    .map(|component_type| {
                        self.components
                            // Get entities => components for this component type.
                            .get(component_type)
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
            };

            // Find the intersection of the previous.
            // This is the set of entities that have all the components.
            let entities_with_all_components = {
                puffin::profile_scope!("entities_with_all_components");

                let mut entities_with_each_component =
                    entities_with_each_component.collect::<Vec<_>>();

                let mut ids = vec![];

                for id in entities_with_each_component.remove(0) {
                    let mut in_all = true;

                    for other_ids in &mut entities_with_each_component {
                        let index_option = other_ids.iter().position(|x| *x == id);

                        if let Some(index) = index_option {
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
                            let entities_to_components = &self.components[component_type];
                            // Return the component.
                            &entities_to_components[&id]
                        })
                        // Into a vector.
                        .collect::<Vec<_>>();

                    // This converts a vector to a sized array.
                    let components_array: [&Component; N] =
                        components_vec.try_into().unwrap_or_else(|_| panic!());

                    // Join with the id in a tuple.
                    (id, components_array)
                })
                // Into a vector to evaluate everything.
                .collect::<Vec<(u128, [_; N])>>()
                // Into an iterator for ease of use in a system.
                .into_iter()
        }
    }

    /// Get an iterator over components of given types, in the form [comp1, comp2, comp3].
    #[must_use]
    pub fn query_by_type_id<const N: usize>(
        &self,
        component_types: [&TypeId; N],
    ) -> std::vec::IntoIter<[&Component; N]> {
        // Do the usual query.
        self.query_by_type_id_with_id(component_types)
            // Remove the id.
            .map(|(_, components)| components)
            // Into a vector to evaluate everything.
            .collect::<Vec<_>>()
            // Into an iterator for ease of use in a system.
            .into_iter()
    }

    /// Get system inputs.
    #[must_use]
    pub fn query<'a, Inputs: SystemInputs<'a>>(&'a self) -> std::vec::IntoIter<Inputs> {
        Inputs::from_entities(self)
    }

    /// Get a single component of a given type.
    ///
    /// This gets the first component of the given type,
    ///
    /// but should only be used when you're sure there is only one.
    #[must_use]
    pub fn get<T: ComponentTrait>(&self) -> &mut T {
        let component_type = &TypeId::of::<T>();

        self.get_by_type_id(component_type).get_mut::<T>()
    }

    /// Get a single component with a given type id.
    ///
    /// This gets the first component of the given type,
    ///
    /// but should only be used when you're sure there is only one.
    #[must_use]
    pub fn get_by_type_id(&self, component_type: &TypeId) -> &Component {
        self.components[component_type]
            .values()
            .next()
            .expect("Component not found.")
    }
}
