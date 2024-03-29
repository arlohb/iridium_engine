//! Provides utilities for creating a `HashMap` with various things applied to the values.

/// A macro to create a `HashMap` with nicer syntax.
///
/// # Examples
///
/// ```
/// # use iridium_map_utils::fast_map;
/// let map = fast_map! {
///    "key" => "value",
/// };
/// ```
#[macro_export]
macro_rules! fast_map {
    ($($key:expr => $value:expr),* $(,)*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key.to_owned(), $value);
            )*
            map
        }
    };
}

/// Like `fast_map!`, but wraps the values in a `Box<dyn Any>`.
#[macro_export]
macro_rules! fast_map_any {
    ($($key:expr => $value:expr),* $(,)*) => {
        {
            let mut map = std::collections::HashMap::<String, Box<dyn std::any::Any>>::new();
            $(
                map.insert($key.to_owned(), Box::new($value));
            )*
            map
        }
    };
}
