use hashbrown::HashMap;
use std::{any::Any, sync::Arc};

use crate::Asset;

/// The asset manager to store all assets such as textures, shaders, etc.
pub struct Assets {
    assets: HashMap<String, Arc<dyn Any + Send + Sync>>,
}

impl Default for Assets {
    fn default() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }
}

impl Assets {
    /// Creates a new asset manager.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an asset.
    pub fn add<T: Any + Send + Sync + 'static>(&mut self, id: &str, asset: T) {
        self.assets.insert(id.to_string(), Arc::new(asset));
    }

    /// Gets an asset.
    #[must_use]
    pub fn get<T: Any + Send + Sync + 'static>(&self, id: &str) -> Option<Asset<T>> {
        self.assets
            .get(id)
            .map(|asset| Asset::<T>::from_arc_any(asset.clone()))
    }
}
