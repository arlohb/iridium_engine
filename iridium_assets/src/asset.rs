use std::{sync::Arc, any::Any, ops::Deref};

/// An asset.
pub struct Asset<T: 'static> {
    asset: Arc<dyn Any>,
    phantom: std::marker::PhantomData<*const T>,
}

unsafe impl<T: 'static> Send for Asset<T> {}
unsafe impl<T: 'static> Sync for Asset<T> {}

impl<T> Clone for Asset<T> {
    fn clone(&self) -> Self {
        Self {
            asset: self.asset.clone(),
            phantom: self.phantom,
        }
    }
}

impl<T> Deref for Asset<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> Asset<T> {
    /// Creates a new asset.
    pub fn from_arc_any(asset: Arc<dyn Any>) -> Self {
        Self {
            asset,
            phantom: std::marker::PhantomData,
        }
    }

    /// Gets the asset.
    pub fn get(&self) -> &T {
        self.asset.downcast_ref::<T>().unwrap()
    }
}
