use super::*;
use std::sync::{Mutex, MutexGuard};
use hashbrown::HashMap;

/// Stores all the entities in the scene.
pub struct Entities {
    /// entity_id => components
    entities: HashMap<u128, Vec<String>>,
    /// component_type => entity_id => component
    components: HashMap<String, HashMap<u128, Mutex<Component>>>,
    pub component_types: HashMap<String, ComponentType>,
}

impl Entities {
    /// Create a new empty instance.
    pub fn new(component_types: Vec<HashMap<String, ComponentType>>) -> Entities {
        let component_types = component_types
            .into_iter()
            .fold(HashMap::new(), |mut acc, map| {
                acc.extend(map);
                acc
            });

        Entities {
            entities: HashMap::new(),
            components: HashMap::new(),
            component_types,
        }
    }

    /// Add components to an entity.
    pub fn add_components(&mut self, entity_id: u128, components: Vec<Component>) {
        // If the entity doesn't exist,
        if !self.entities.contains_key(&entity_id) {
            // Add it.
            self.entities.insert(entity_id, vec![]);
        }

        // For each component to be added.
        for component in components {
            // Add to entities.
            self.entities.get_mut(&entity_id).unwrap().push(component.name.clone());

            // If components of this type already exists,
            if self.components.contains_key(&component.name) {
                // Add to it.
                self.components.get_mut(&component.name).unwrap().insert(entity_id, Mutex::new(component));
            } else {
                // Create a new one.
                let name = component.name.clone();
                let mut components = HashMap::new();
                // And insert the component into it.
                components.insert(entity_id, Mutex::new(component));
                self.components.insert(name, components);
            }
        }
    }

    /// Create a new entity with the given components.
    /// Automatically adds the Name component with the given name.
    pub fn new_entity(&mut self, name: &str, components: Vec<Component>) -> u128 {
        // Generate a new entity id.
        let id = uuid::Uuid::new_v4().as_u128();

        // Add it to entities.
        self.entities.insert(id, vec![]);

        // Add the name component.
        self.add_components(id, vec![
            create_component! { Name
                name: name.to_owned(),
            }
        ]);

        // Add the other components.
        self.add_components(id, components);

        // Return the id.
        id
    }

    /// Get all the component types an entity has.
    pub fn get_entity_component_types(&self, entity_id: u128) -> Vec<String> {
        // Do a simple look up in the entities map.
        self.entities.get(&entity_id).unwrap().clone()
    }

    /// Get all the components of a given entity.
    pub fn get_entity_components(&self, entity_id: u128) -> Vec<MutexGuard<Component>> {
        // Get the component types of the entity.
        let component_types = self.get_entity_component_types(entity_id);

        // For each component the entity has.
        component_types.into_iter().map(|component_type| {
            // Get the map of entities => components.
            let entities_to_components = self.components.get(&component_type).unwrap();
            // Get the component for this entity.
            let component_mutex = entities_to_components.get(&entity_id).unwrap();

            // Lock the Component.
            let component = component_mutex.lock().unwrap();

            // Return the component.
            component
        // Collect into a HashMap
        }).collect::<Vec<_>>()
    }

    /// Get an iterator over components of given types, in the form (entity_id, [comp1, comp2, comp3]).
    pub fn query_with_id<const N: usize>(
        &self, component_types: [&str; N]
    ) -> std::vec::IntoIter<(u128, [MutexGuard<component::Component>; N])> {
        // Find all the entities that have each component.
        let entities_with_each_component = component_types.into_iter()
            // Get all the entities that have each component.
            .map(|component_type| self.components
                // Get entities => components for this component type.
                .get(component_type)
                // Do this if previous is Some.
                .map(|map| map
                    // Only get the entity id.
                    .keys()
                    // From &u128 -> u128
                    .copied()
                    // Into a vector.
                    .collect::<Vec<u128>>()
                )
                // If previous is None, return an empty vector.
                .or_else(|| Some(vec![]))
                // This is now definitely Some.
                .unwrap()
            );

        // Find the intersection of the previous.
        // This is the set of entities that have all the components.
        let entities_with_all_components = entities_with_each_component
            .reduce(|result, current| result.into_iter()
                // Intersect the previous result with the current result.
                .filter(|id| current.contains(id))
                // Into a vector.
                .collect::<Vec<_>>()
            )
            // We know previous was not empty, so this is definitely Some.
            .unwrap();

        // Create the final return value for each entity previously found.
        entities_with_all_components.iter()
            // For each entity id.
            .map(|id| {
                // This converts a vector to a sized array.
                fn into_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
                    v.try_into().unwrap_or_else(|_| panic!())
                }

                // Get the given components for the entity.
                let components_vec = component_types.into_iter()
                    // For each component type.
                    .map(|component_type| {
                        // Get the map of entities => components.
                        let entities_to_components = &self.components[component_type];
                        // Get the component for this entity.
                        let component_mutex = &entities_to_components[id];

                        // Lock the component.
                        component_mutex.lock().unwrap()
                    })
                    // Into a vector.
                    .collect::<Vec<_>>();

                // Convert this to an array.
                let components_array = into_array(components_vec);

                // Join with the id in a tuple.
                (
                    *id,
                    components_array,
                )
            })
            // Into a vector to evaluate everything.
            .collect::<Vec<(u128, [_; N])>>()
            // Into an iterator for ease of use in a system.
            .into_iter()
    }

    /// Get an iterator over components of given types, in the form [comp1, comp2, comp3].
    pub fn query<const N: usize>(
        &self, component_types: [&str; N]
    ) -> std::vec::IntoIter<[MutexGuard<component::Component>; N]> {
        // Do the usual query.
        self.query_with_id(component_types)
            // Remove the id.
            .map(|(_, components)| components)
            // Into a vector to evaluate everything.
            .collect::<Vec<_>>()
            // Into an iterator for ease of use in a system.
            .into_iter()
    }

    /// Get a single component of a given type.
    /// This gets the first component of the given type,
    /// but should only be used when you're sure there is only one.
    pub fn get(
        &self, component_type: &str
    ) -> MutexGuard<component::Component> {
        // Do a usual query.
        self.query([component_type])
            // Get the first component.
            .next()
            // If there isn't one, panic.
            .unwrap()
            // Convert the array of length 1 to a single element.
            .into_iter()
            .next().unwrap()
    }
}
