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

    pub fn query_1<T>(&mut self) -> Vec<&mut Entity>
        where T: Component
    {
        self.entities
            .iter_mut()
            .filter(|e|
                e.components.iter().any(|c| c.dyn_get_type() == T::get_type())
            )
            .collect()
    }

    pub fn query_2<T, U>(&mut self) -> Vec<&mut Entity>
        where T: Component,
              U: Component
    {
        self.entities
            .iter_mut()
            .filter(|e|
                e.components.iter().any(|c| c.dyn_get_type() == T::get_type())
                && e.components.iter().any(|c| c.dyn_get_type() == U::get_type())
            )
            .collect()
    }

    pub fn query_3<T, U, V>(&mut self) -> Vec<&mut Entity>
        where T: Component,
              U: Component,
              V: Component
    {
        self.entities
            .iter_mut()
            .filter(|e|
                e.components.iter().any(|c| c.dyn_get_type() == T::get_type())
                && e.components.iter().any(|c| c.dyn_get_type() == U::get_type())
                && e.components.iter().any(|c| c.dyn_get_type() == V::get_type())
            )
            .collect()
    }
}
