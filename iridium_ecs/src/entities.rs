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
        where T: Component + Default
    {
        let t = T::default();
        self.entities
            .iter_mut()
            .filter(|e|
                e.components.iter().any(|c| c.get_type() == t.get_type())
            )
            .collect()
    }

    pub fn query_2<T, U>(&mut self) -> Vec<&mut Entity>
        where T: Component + Default,
              U: Component + Default
    {
        let t = T::default();
        let u = U::default();
        self.entities
            .iter_mut()
            .filter(|e|
                e.components.iter().any(|c| c.get_type() == t.get_type())
                && e.components.iter().any(|c| c.get_type() == u.get_type())
            )
            .collect()
    }

    pub fn query_3<T, U, V>(&mut self) -> Vec<&mut Entity>
        where T: Component + Default,
              U: Component + Default,
              V: Component + Default
    {
        let t = T::default();
        let u = U::default();
        let v = V::default();
        self.entities
            .iter_mut()
            .filter(|e|
                e.components.iter().any(|c| c.get_type() == t.get_type())
                && e.components.iter().any(|c| c.get_type() == u.get_type())
                && e.components.iter().any(|c| c.get_type() == v.get_type())
            )
            .collect()
    }
}
