use super::*;
use std::sync::{Mutex, MutexGuard};
use hashbrown::HashMap;

pub struct Entities {
    entities: Vec<u128>,
    /// component_type => entity_id => component
    components: HashMap<String, HashMap<u128, Mutex<Component>>>,
    pub component_types: HashMap<String, ComponentType>,
}

impl Entities {
    pub fn new(component_types: HashMap<String, ComponentType>) -> Entities {
        Entities {
            entities: vec![],
            components: HashMap::new(),
            component_types,
        }
    }

    pub fn add_components(&mut self, entity_id: u128, components: Vec<Component>) {
        components.into_iter().for_each(|component| {
            match self.components.contains_key(&component.name) {
                true => {
                    self.components.get_mut(&component.name).unwrap().insert(entity_id, Mutex::new(component));
                }
                false => {
                    let name = component.name.clone();
                    let mut components = HashMap::new();
                    components.insert(entity_id, Mutex::new(component));
                    self.components.insert(name, components);
                }
            }
        })
    }

    pub fn new_entity(&mut self, name: &str, components: Vec<Component>) -> u128 {
        let id = uuid::Uuid::new_v4().as_u128();

        self.entities.push(id);

        self.add_components(id, vec![
            create_component! { Name
                name: name.to_owned(),
            }
        ]);

        self.add_components(id, components);

        id
    }

    pub fn query_with_id<const N: usize>(
        &self, component_types: [&str; N]
    ) -> std::vec::IntoIter<(u128, [MutexGuard<component::Component>; N])> {
        component_types
            .iter()
            // for each component_type, get a list of entities that have that component
            .map(|component_type| 
                self.components
                    .get(*component_type)
                    .map(|some| some
                        .keys()
                        .copied()
                        .collect::<Vec<u128>>())
                    .or_else(|| Some(vec![]))
                    .unwrap())
            // reduce the lists to the intersection of all these lists
            .reduce(|result, current| result
                .iter()
                .copied()
                .filter(|id| current.contains(id))
                .collect::<Vec<u128>>())
            .unwrap()
            .iter()
            .map(|id| {
                fn into_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
                    v.try_into().unwrap_or_else(|_| panic!())
                }

                (
                    *id,
                    into_array(component_types
                    .iter()
                    .map(|name| self.components
                        [*name]
                        [id]
                        .lock()
                        .unwrap())
                    .collect::<Vec<_>>())
                )
                })
            .collect::<Vec<(u128, [_; N])>>()
            .into_iter()
    }

    pub fn query<const N: usize>(
        &self, component_types: [&str; N]
    ) -> std::vec::IntoIter<[MutexGuard<component::Component>; N]> {
        self.query_with_id(component_types)
            .map(|(_, components)| components)
            .collect::<Vec<_>>()
            .into_iter()
    }
}
