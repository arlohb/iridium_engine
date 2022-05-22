use std::{any::Any, sync::Arc};

use hashbrown::HashMap;

pub struct ComponentType {
    pub name: String,
    pub values: HashMap<String, String>,
}

pub struct Component {
    pub name: String,
    pub schema: Arc<ComponentType>,
    values: HashMap<String, Box<dyn Any>>,
}

impl std::fmt::Debug for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{ ", self.name)?;
        for (key, value_type) in &self.schema.values {
            let value = self.values.get(key).unwrap();
            match value_type.as_str() {
                "f32" => write!(f, "{}: {:?}, ", key, value.downcast_ref::<f32>().unwrap())?,
                "f64" => write!(f, "{}: {:?}, ", key, value.downcast_ref::<f64>().unwrap())?,
                "i32" => write!(f, "{}: {:?}, ", key, value.downcast_ref::<i32>().unwrap())?,
                "i64" => write!(f, "{}: {:?}, ", key, value.downcast_ref::<i64>().unwrap())?,
                "u32" => write!(f, "{}: {:?}, ", key, value.downcast_ref::<u32>().unwrap())?,
                "u64" => write!(f, "{}: {:?}, ", key, value.downcast_ref::<u64>().unwrap())?,
                "bool" => write!(f, "{}: {:?}, ", key, value.downcast_ref::<bool>().unwrap())?,
                _ => write!(f, "{}: {:?}, ", key, value_type)?,
            }
        }
        write!(f, "}}")
    }
}

impl Component {
    pub fn new(
        name: &str,
        schema: Arc<ComponentType>,
        values: HashMap<String, Box<dyn Any>>,
    ) -> Component {
        Component {
            name: name.to_owned(),
            schema,
            values,
        }
    }

    pub fn get<T>(&self, key: &str) -> Option<&T>
        where T: 'static {
        let value = self.values.get(&key.to_owned())?;
        value.downcast_ref::<T>()
    }

    pub fn get_mut<T>(&mut self, key: &str) -> Option<&mut T>
        where T: 'static {
        let value = self.values.get_mut(&key.to_owned())?;
        value.downcast_mut::<T>()
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
    ($($key:expr => $schema:expr => $value:expr),*) => {
        {
            let mut components = Vec::new();
            $(
                components.push(Component::new(
                    $key,
                    $schema.clone(),
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
            let mut components = hashbrown::HashMap::<String, std::sync::Arc<ComponentType>>::new();
            $(
                components.insert(
                    $key.to_string(),
                    std::sync::Arc::new(ComponentType {
                        name: $key.to_string(),
                        values: $value
                    })
                );
            )*
            components
        }
    };
}
