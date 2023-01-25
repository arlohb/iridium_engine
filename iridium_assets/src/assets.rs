use std::collections::HashMap;
use std::{
    any::Any,
    sync::{Arc, RwLock},
};

use crate::Asset;

/// The asset manager to store all assets such as textures, shaders, etc.
#[derive(Default)]
pub struct Assets {
    assets: HashMap<String, Arc<RwLock<dyn Any + Send + Sync>>>,
}

impl Assets {
    /// Creates a new asset manager.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an asset.
    pub fn add<T: Any + Send + Sync + 'static>(&mut self, id: &str, asset: T) {
        self.assets
            .insert(id.to_string(), Arc::new(RwLock::new(asset)));
    }

    /// Gets an asset.
    #[must_use]
    pub fn get<T: Any + Send + Sync + 'static>(&self, id: &str) -> Option<Asset<T>> {
        self.assets
            .get(id)
            .map(|asset| Asset::<T>::from_inner(id.to_string(), asset.clone()))
    }

    /// Gets all assets.
    #[must_use]
    pub fn get_all(&self) -> Vec<(String, Arc<RwLock<dyn Any + Send + Sync>>)> {
        self.assets
            .iter()
            .map(|(id, asset)| (id.clone(), asset.clone()))
            .collect()
    }
}
