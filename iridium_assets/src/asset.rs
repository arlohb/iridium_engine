use std::{any::Any, cell::UnsafeCell, ops::Deref, sync::Arc};

use iridium_reflect::{HasStableTypeId, StableTypeId};

use crate::Assets;

/// Indicates a type can be stored and referenced as an asset.
pub trait Asset: Any + Send + Sync + HasStableTypeId {}

/// How an asset is stored internally.
pub type RawAsset = Arc<UnsafeCell<dyn Asset>>;

/// An asset.
pub struct AssetBox<T: Asset> {
    /// The ID of the asset.
    id: String,
    /// If the ID has changed but the asset hasn't.
    invalid: bool,

    asset: RawAsset,
    phantom: std::marker::PhantomData<*const T>,
}

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl<T: Asset> Send for AssetBox<T> {}
unsafe impl<T: Asset> Sync for AssetBox<T> {}

impl<T: Asset> Clone for AssetBox<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            asset: self.asset.clone(),
            invalid: self.invalid,
            phantom: self.phantom,
        }
    }
}

impl<T: Asset> Deref for AssetBox<T> {
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

impl<T: Asset> AssetBox<T> {
    /// Creates a new asset.
    ///
    /// # Errors
    ///
    /// If the asset is not the correct type.
    pub fn from_inner(id: String, asset: RawAsset) -> Result<Self, String> {
        if T::stable_type_id() != Self::stable_type_id_from_inner(&asset) {
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
    pub fn stable_type_id_from_inner(inner: &RawAsset) -> StableTypeId {
        unsafe { &*inner.get() }.dyn_stable_type_id()
    }

    /// Get the type id of the asset.
    ///
    /// If `Asset` is used properly this should always correspond with T.
    #[must_use]
    pub fn stable_type_id(&self) -> StableTypeId {
        Self::stable_type_id_from_inner(&self.asset)
    }

    /// Checks whether the asset has the given type id.
    #[must_use]
    pub fn is_type(&self, other: StableTypeId) -> bool {
        self.stable_type_id() == other
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
        let ptr = self.asset.get();

        #[allow(clippy::ptr_as_ptr)]
        self.is_type(T::stable_type_id())
            .then(|| unsafe { &*(ptr as *const _ as *const T) })
            .ok_or_else(|| "Asset type mismatch".to_string())
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
        let ptr = self.asset.get();

        #[allow(clippy::ptr_as_ptr)]
        self.is_type(T::stable_type_id())
            .then(|| unsafe { &mut *(ptr as *mut _ as *mut T) })
            .ok_or_else(|| "Asset type mismatch".to_string())
    }

    /// Replace the id of the asset.
    /// This won't load the new asset immediately,
    /// only when `Self::update_asset` is called,
    /// which should be done automatically by the engine.
    pub fn change_id(&mut self, new_id: String) {
        println!("Asset id changed to {new_id}");
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

        println!("Asset with id '{id}' updating", id = self.id);

        let new_asset = assets.get::<T>(&self.id)?;

        if self.is_type(new_asset.stable_type_id()) {
            *self = new_asset;
            Ok(true)
        } else {
            Err(format!("New asset '{}' has wrong type", self.id))
        }
    }
}
