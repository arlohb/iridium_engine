use std::{
    any::Any,
    fmt::Write,
};

use hashbrown::HashMap;

pub struct ComponentType {
    pub name: String,
    pub values: HashMap<String, String>,
}

pub struct Component {
    pub name: String,
    values: HashMap<String, Box<dyn Any>>,
}

unsafe impl Send for Component {}
unsafe impl Sync for Component {}

impl Component {
    pub fn new(
        name: &str,
        values: HashMap<String, Box<dyn Any>>,
    ) -> Component {
        Component {
            name: name.to_owned(),
            values,
        }
    }

    pub fn display(&self, component_type: &ComponentType) -> String {
        let mut result = String::new();
        write!(result, "{} {{ ", self.name).unwrap();
        for (key, value_type) in &component_type.values {
            let value = self.values.get(key).unwrap();
            match value_type.as_str() {
                "f32" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<f32>().unwrap()).unwrap(),
                "f64" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<f64>().unwrap()).unwrap(),
                "i32" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<i32>().unwrap()).unwrap(),
                "i64" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<i64>().unwrap()).unwrap(),
                "u32" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<u32>().unwrap()).unwrap(),
                "u64" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<u64>().unwrap()).unwrap(),
                "bool" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<bool>().unwrap()).unwrap(),
                "String" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<String>().unwrap()).unwrap(),
                _ => write!(result, "{}: {:?}, ", key, value_type).unwrap(),
            }
        }
        write!(result, "}}").unwrap();
        result
    }

    pub fn get<T>(&self, key: &str) -> &T
        where T: 'static {
        let value = self.values.get(&key.to_owned()).unwrap();
        value.downcast_ref::<T>().unwrap()
    }

    pub fn get_mut<T>(&mut self, key: &str) -> &mut T
        where T: 'static {
        let value = self.values.get_mut(&key.to_owned()).unwrap();
        value.downcast_mut::<T>().unwrap()
    }

    pub fn add<T>(&mut self, key: &str, value: T)
        where T: 'static {
        self.values.insert(key.to_owned(), Box::new(value));
    }
}

#[macro_export]
macro_rules! fast_map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = hashbrown::HashMap::new();
            $(
                map.insert($key.to_owned(), $value.to_owned());
            )*
            map
        }
    };
}

#[macro_export]
macro_rules! fast_map_any {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = hashbrown::HashMap::<String, Box<dyn std::any::Any>>::new();
            $(
                map.insert($key.to_owned(), Box::new($value));
            )*
            map
        }
    };
}

#[macro_export]
macro_rules! create_components {
    ($($key:expr => $value:expr),*) => {
        {
            let mut components = Vec::<Component>::new();
            $(
                components.push(Component::new(
                    $key,
                    $value
                ));
            )*
            components
        }
    };
}

#[macro_export]
macro_rules! create_component_types {
    ($($key:expr => $value:expr),*) => {
        {
            let mut components = hashbrown::HashMap::<String, ComponentType>::new();
            $(
                components.insert(
                    $key.to_string(),
                    ComponentType {
                        name: $key.to_string(),
                        values: $value
                    }
                );
            )*
            components
        }
    };
}
