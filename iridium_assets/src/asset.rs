use std::{
    any::{Any, TypeId},
    ops::Deref,
    sync::{Arc, RwLock},
};

use crate::Assets;

/// How an asset is stored internally.
pub type RawAsset = Arc<RwLock<dyn Any + Send + Sync>>;

/// An asset.
pub struct Asset<T: Any + Send + Sync> {
    /// The ID of the asset.
    id: String,
    /// If the ID has changed but the asset hasn't.
    invalid: bool,

    asset: RawAsset,
    phantom: std::marker::PhantomData<*const T>,
}

unsafe impl<T: Any + Send + Sync> Send for Asset<T> {}
unsafe impl<T: Any + Send + Sync> Sync for Asset<T> {}

impl<T: Any + Send + Sync> Clone for Asset<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            asset: self.asset.clone(),
            invalid: self.invalid,
            phantom: self.phantom,
        }
    }
}

impl<T: Any + Send + Sync> Deref for Asset<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.get() {
            Ok(t) => t,
            // I'm fine with panicking here,
            // because if `Asset` is used properly, `Asset::get` shouldn't fail,
            // and `Asset` not being used properly should be a serious error,
            // and not something to be fixed at runtime.
            Err(err) => panic!("Asset deref failed with error: {err}"),
        }
    }
}

impl<T: Any + Send + Sync> Asset<T> {
    /// Creates a new asset.
    ///
    /// # Errors
    ///
    /// If the asset is not the correct type.
    pub fn from_inner(id: String, asset: RawAsset) -> Result<Self, String> {
        if TypeId::of::<T>() != Self::type_id_from_inner(&asset) {
            return Err("Asset was not of type `T`".into());
        }

        Ok(Self {
            id,
            asset,
            invalid: false,
            phantom: std::marker::PhantomData,
        })
    }

    /// Get the type id of an inner asset.
    #[must_use]
    pub fn type_id_from_inner(inner: &RawAsset) -> TypeId {
        let any = inner.read().expect("Asset RwLock poisoned");
        (*any).type_id()
    }

    /// Get the type id of the asset.
    ///
    /// If `Asset` is used properly this should always correspond with T.
    #[must_use]
    pub fn type_id(&self) -> TypeId {
        Self::type_id_from_inner(&self.asset)
    }

    /// Checks whether the asset has the given type id.
    #[must_use]
    pub fn is_type(&self, other: TypeId) -> bool {
        self.type_id() == other
    }

    /// Gets the id.
    ///
    /// This only reflects the actual asset if `invalid` isn't set.
    /// If used in the engine in a component,
    /// this should never be invalid.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Gets the invalid state.
    ///
    /// This means a new asset id hasn't been loaded.
    #[must_use]
    pub const fn invalid(&self) -> bool {
        self.invalid
    }

    /// Gets the asset.
    ///
    /// # Errors
    ///
    /// If the asset is not of the expected type,
    /// or the inner `RwLock` has been poisoned.
    pub fn get(&self) -> Result<&T, String> {
        let dyn_guard = self
            .asset
            .read()
            .map_err(|_| "Asset RwLock poisoned".to_string())?;

        let local_ref = dyn_guard
            .downcast_ref::<T>()
            .ok_or_else(|| "Asset type mismatch".to_string())?;

        unsafe {
            Ok(std::mem::transmute::<&T, & /*'self*/ T>(local_ref))
        }
    }

    /// Mutably gets the asset.
    /// This will block the current thread until the asset is available.
    ///
    /// # Errors
    ///
    /// If the asset is not of the expected type,
    /// or the inner `RwLock` has been poisoned.
    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> Result<&mut T, String> {
        let mut dyn_guard = self
            .asset
            .write()
            .map_err(|_| "Asset RwLock poisoned".to_string())?;

        let local_mut = dyn_guard
            .downcast_mut::<T>()
            .ok_or_else(|| "Asset type mismatch".to_string())?;

        unsafe {
            Ok(std::mem::transmute::<&mut T, & /*'self*/ mut T>(local_mut))
        }
    }

    /// Replace the id of the asset.
    /// This won't load the new asset immediately,
    /// only when `Self::update_asset` is called,
    /// which should be done automatically by the engine.
    pub fn change_id(&mut self, new_id: String) {
        self.id = new_id;
        self.invalid = true;
    }

    /// Updates the assets.
    /// This will check whether the asset has been invalidated,
    /// and early return if it's still valid.
    ///
    /// Returns true if the asset was reloaded, false if not.
    ///
    /// # Errors
    ///
    /// If the asset isn't found or the new asset type is wrong
    pub fn update_asset(&mut self, assets: &Assets) -> Result<bool, String> {
        if !self.invalid {
            return Ok(false);
        }

        let new_asset = assets.get::<T>(&self.id)?;

        if self.is_type(new_asset.type_id()) {
            *self = new_asset;
            Ok(true)
        } else {
            Err(format!("New asset '{}' has wrong type", self.id))
        }
    }
}
