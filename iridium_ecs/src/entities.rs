use super::*;

pub struct Entities {
    entities: Vec<Entity>,
}

impl Entities {
    pub fn new(entities: Vec<Entity>) -> Entities {
        Entities {
            entities,
        }
    }

    pub fn get(&self, id: u128) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == id)
    }

    pub fn query(&mut self, names: Vec<&str>) -> Vec<&mut Entity> {
        self.entities
            .iter_mut()
            .filter(|e|
                names.iter().all(|name|
                    e.components.iter().any(|comp| comp.name == *name)
                )
            )
            .collect()
    }
}
