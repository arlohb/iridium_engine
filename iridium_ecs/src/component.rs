use std::{
    any::Any,
    fmt::Write,
};

use hashbrown::HashMap;

#[derive(Clone)]
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
macro_rules! create_component {
    (
        $name:ident
        $($key:ident: $value:expr),* $(,)?
    ) => {
        {
            let mut values = hashbrown::HashMap::<String, Box<dyn std::any::Any>>::new();
            $(
                values.insert(stringify!($key).to_string(), Box::new($value));
            )*

            $crate::Component::new(
                stringify!($name),
                values,
            )
        }
    };
}

#[macro_export]
macro_rules! create_component_type {
    (
        struct $name:ident {
            $($key:ident: $value_type:ty),* $(,)?
        }
    ) => {
        {
            let mut value_types = hashbrown::HashMap::new();
            $(
                value_types.insert(
                    stringify!($key).to_string(),
                    stringify!($value_type).to_string()
                );
            )*

            $crate::ComponentType {
                name: stringify!($name).to_string(),
                values: value_types,
            }
        }
    };
}

#[macro_export]
macro_rules! create_component_types {
    (
        $(struct $name:ident {
            $($key:ident: $value_type:ty),* $(,)?
        }),* $(,)*
    ) => {
        {
            let mut component_types = hashbrown::HashMap::<String, $crate::ComponentType>::new();
            $(
                let mut value_types = hashbrown::HashMap::new();
                $(
                    value_types.insert(
                        stringify!($key).to_string(),
                        stringify!($value_type).to_string()
                    );
                )*
                component_types.insert(
                    stringify!($name).to_string(),
                    $crate::ComponentType {
                        name: stringify!($name).to_string(),
                        values: value_types,
                    },
                );
            )*
            component_types
        }
    };
}
