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
macro_rules! fast_map_arc {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = hashbrown::HashMap::new();
            $(
                map.insert($key.to_owned(), std::sync::Arc::new($value));
            )*
            map
        }
    };
}
