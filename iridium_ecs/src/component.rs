#![allow(clippy::mut_from_ref)]

use std::cell::UnsafeCell;

pub trait ComponentTrait: 'static + Send + Sync {
    fn type_name() -> &'static str where Self: Sized;
    fn dyn_type_name(&self) -> &'static str;
}

pub struct Component {
    data: Box<UnsafeCell<dyn ComponentTrait>>,
}

unsafe impl Send for Component {}
unsafe impl Sync for Component {}

impl Component {
    pub fn new<T>(component: T) -> Self
    where T: ComponentTrait + 'static {
        Component {
            data: Box::new(UnsafeCell::new(component)),
        }
    }

    pub fn get_dyn(&self) -> &dyn ComponentTrait {
        unsafe {
            &*self.data.get()
        }
    }

    pub fn get_dyn_mut(&self) -> &mut dyn ComponentTrait {
        unsafe {
            &mut *self.data.get()
        }
    }

    pub fn get<T: ComponentTrait>(&self) -> &T {
        unsafe {
            &*(self.data.get() as *const _ as *const T)
        }
    }

    pub fn get_mut<T: ComponentTrait>(&self) -> &mut T {
        unsafe {
            &mut *(self.data.get() as *mut _ as *mut T)
        }
    }

    pub fn type_name(&self) -> &'static str {
        self.get_dyn().dyn_type_name()
    }
}

// impl Component {
//     pub fn new(
//         name: &str,
//         component: Box<dyn ComponentTrait>,
//     ) -> Component {
//         Component {
//             type_name: name.to_owned(),
//             component,
//         }
//     }

    // pub fn display(&self, component_type: &ComponentType) -> String {
    //     let mut result = String::new();
    //     write!(result, "{} {{ ", self.type_name).unwrap();
    //     for (key, value_type) in &component_type.values {
    //         let value = self.values.get(key).unwrap();
    //         match value_type.as_str() {
    //             "f32" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<f32>().unwrap()).unwrap(),
    //             "f64" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<f64>().unwrap()).unwrap(),
    //             "i32" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<i32>().unwrap()).unwrap(),
    //             "i64" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<i64>().unwrap()).unwrap(),
    //             "u32" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<u32>().unwrap()).unwrap(),
    //             "u64" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<u64>().unwrap()).unwrap(),
    //             "bool" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<bool>().unwrap()).unwrap(),
    //             "String" => write!(result, "{}: {:?}, ", key, value.downcast_ref::<String>().unwrap()).unwrap(),
    //             _ => write!(result, "{}: {:?}, ", key, value_type).unwrap(),
    //         }
    //     }
    //     write!(result, "}}").unwrap();
    //     result
    // }

    // pub fn get<T>(&self, key: &str) -> &T
    //     where T: 'static {
    //     let value = self.values.get(&key.to_owned()).unwrap();
    //     value.downcast_ref::<T>().unwrap()
    // }

    // pub fn get_mut<T>(&mut self, key: &str) -> &mut T
    //     where T: 'static {
    //     let value = self.values.get_mut(&key.to_owned()).unwrap();
    //     value.downcast_mut::<T>().unwrap()
    // }

    // pub fn add<T>(&mut self, key: &str, value: T)
    //     where T: 'static {
    //     self.values.insert(key.to_owned(), Box::new(value));
    // }
// }

// #[macro_export]
// macro_rules! create_component {
//     (
//         $name:ident
//         $($key:ident: $value:expr),* $(,)?
//     ) => {
//         {
//             let mut values = hashbrown::HashMap::<String, Box<dyn std::any::Any>>::new();
//             $(
//                 values.insert(stringify!($key).to_string(), Box::new($value));
//             )*

//             $crate::Component::new(
//                 stringify!($name),
//                 values,
//             )
//         }
//     };
// }

// #[macro_export]
// macro_rules! create_component_type {
//     (
//         struct $name:ident {
//             $($key:ident: $value_type:ty),* $(,)?
//         }
//     ) => {
//         {
//             let mut value_types = hashbrown::HashMap::new();
//             $(
//                 value_types.insert(
//                     stringify!($key).to_string(),
//                     stringify!($value_type).to_string()
//                 );
//             )*

//             $crate::ComponentType {
//                 name: stringify!($name).to_string(),
//                 values: value_types,
//             }
//         }
//     };
// }

// #[macro_export]
// macro_rules! create_component_types {
//     (
//         $(struct $name:ident {
//             $($key:ident: $value_type:ty),* $(,)?
//         }),* $(,)*
//     ) => {
//         {
//             let mut component_types = hashbrown::HashMap::<String, $crate::ComponentType>::new();
//             $(
//                 let mut value_types = hashbrown::HashMap::new();
//                 $(
//                     value_types.insert(
//                         stringify!($key).to_string(),
//                         stringify!($value_type).to_string()
//                     );
//                 )*
//                 component_types.insert(
//                     stringify!($name).to_string(),
//                     $crate::ComponentType {
//                         name: stringify!($name).to_string(),
//                         values: value_types,
//                     },
//                 );
//             )*
//             component_types
//         }
//     };
// }

pub struct Name {
    pub name: String,
}

impl ComponentTrait for Name {
    fn type_name() -> &'static str { "Name" }
    fn dyn_type_name(&self) -> &'static str { "Name" }
}
