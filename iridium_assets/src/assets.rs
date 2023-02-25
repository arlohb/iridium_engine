use std::collections::HashMap;
use std::{
    any::Any,
    sync::{Arc, RwLock},
};

use crate::{Asset, RawAsset};

/// The asset manager to store all assets such as textures, shaders, etc.
#[derive(Default)]
pub struct Assets {
    assets: HashMap<String, RawAsset>,
}

impl Assets {
    /// Creates a new asset manager.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an asset.
    pub fn add<T: Any + Send + Sync>(&mut self, id: &str, asset: T) {
        self.assets
            .insert(id.to_string(), Arc::new(RwLock::new(asset)));
    }

    /// Gets an asset.
    ///
    /// # Errors
    ///
    /// Returns an error if the asset id is not found.
    pub fn get<T: Any + Send + Sync>(&self, id: &str) -> Result<Asset<T>, String> {
        let inner = self
            .assets
            .get(id)
            .ok_or(format!("Asset {id} is not found"))?;

        Asset::<T>::from_inner(id.to_string(), inner.clone())
    }

    /// Gets all assets.
    #[must_use]
    pub fn get_all(&self) -> Vec<(String, RawAsset)> {
        self.assets
            .iter()
            .map(|(id, asset)| (id.clone(), asset.clone()))
            .collect()
    }
}
