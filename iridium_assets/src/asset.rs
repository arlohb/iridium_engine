use std::{
    any::Any,
    ops::Deref,
    sync::{Arc, RwLock},
};

/// An asset.
pub struct Asset<T: Send + Sync + 'static> {
    /// The ID of the asset.
    pub id: String,
    asset: Arc<RwLock<dyn Any + Send + Sync>>,
    phantom: std::marker::PhantomData<*const T>,
}

unsafe impl<T: Send + Sync + 'static> Send for Asset<T> {}
unsafe impl<T: Send + Sync + 'static> Sync for Asset<T> {}

impl<T: Send + Sync + 'static> Clone for Asset<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            asset: self.asset.clone(),
            phantom: self.phantom,
        }
    }
}

impl<T: Send + Sync + 'static> Deref for Asset<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T: Send + Sync + 'static> Asset<T> {
    /// Creates a new asset.
    #[must_use]
    pub fn from_inner(id: String, asset: Arc<RwLock<dyn Any + Send + Sync>>) -> Self {
        Self {
            id,
            asset,
            phantom: std::marker::PhantomData,
        }
    }

    /// Gets the asset.
    ///
    /// # Panics
    ///
    /// Panics if the asset is not of the expected type,
    /// or the inner `RwLock` has been poisoned.
    #[must_use]
    pub fn get(&self) -> &T {
        let dyn_guard = self.asset.read().expect("Asset RwLock poisoned");
        let local_ref = dyn_guard
            .downcast_ref::<T>()
            .expect("Asset type mismatch");
        unsafe { std::mem::transmute::<&T, &/*'self*/ T>(local_ref) }
    }

    /// Mutably gets the asset.
    /// This will block the current thread until the asset is available.
    ///
    /// # Panics
    ///
    /// Panics if the asset is not of the expected type,
    /// or the inner `RwLock` has been poisoned.
    #[allow(clippy::mut_from_ref)]
    #[must_use]
    pub fn get_mut(&self) -> &mut T {
        let mut dyn_guard = self.asset.write().expect("Asset RwLock poisoned");
        let local_mut = dyn_guard.downcast_mut::<T>().expect("Asset type mismatch");
        unsafe { std::mem::transmute::<&mut T, &/*'self*/ mut T>(local_mut) }
    }
}
