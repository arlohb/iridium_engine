use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::sync::Arc;

use crate::{Asset, AssetBox, RawAsset};

/// The asset manager to store all assets such as textures, shaders, etc.
#[derive(Default)]
pub struct Assets {
    assets: HashMap<String, RawAsset>,
}

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for Assets {}
unsafe impl Sync for Assets {}

impl Assets {
    /// Creates a new asset manager.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an asset.
    pub fn add<T: Asset>(&mut self, id: &str, asset: T) {
        self.assets
            .insert(id.to_string(), Arc::new(UnsafeCell::new(asset)));
    }

    /// Gets an asset.
    ///
    /// # Errors
    ///
    /// Returns an error if the asset id is not found.
    pub fn get<T: Asset>(&self, id: &str) -> Result<AssetBox<T>, String> {
        let inner = self
            .assets
            .get(id)
            .ok_or(format!("Asset {id} is not found"))?;

        AssetBox::<T>::from_inner(id.to_string(), inner.clone())
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
