use std::{any::Any, ops::Deref, sync::Arc};

/// An asset.
pub struct Asset<T: Send + Sync + 'static> {
    asset: Arc<dyn Any + Send + Sync>,
    phantom: std::marker::PhantomData<*const T>,
}

unsafe impl<T: Send + Sync + 'static> Send for Asset<T> {}
unsafe impl<T: Send + Sync + 'static> Sync for Asset<T> {}

impl<T: Send + Sync + 'static> Clone for Asset<T> {
    fn clone(&self) -> Self {
        Self {
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
    pub fn from_arc_any(asset: Arc<dyn Any + Send + Sync>) -> Self {
        Self {
            asset,
            phantom: std::marker::PhantomData,
        }
    }

    /// Gets the asset.
    ///
    /// # Panics
    ///
    /// Panics if the asset is not of the expected type.
    #[must_use]
    pub fn get(&self) -> &T {
        self.asset
            .downcast_ref::<T>()
            .expect("Asset is not of the expected type")
    }
}
