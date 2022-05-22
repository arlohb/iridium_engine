use super::*;

pub struct Entity {
    pub id: u128,
    pub components: Vec<Component>,
}

impl Entity {
    pub fn new(components: Vec<Component>) -> Entity {
        Entity {
            id: 0,
            components,
        }
    }

    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    pub fn get_component(&self, name: &str) -> Option<&Component> {
        self.components.iter().find(|comp| {
            comp.name == name
        })
    }

    pub fn get_component_mut(&mut self, name: &str) -> Option<&mut Component> {
        self.components.iter_mut().find(|comp| {
            comp.name == name
        })
    }
}
