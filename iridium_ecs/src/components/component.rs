#![allow(clippy::mut_from_ref)]

use std::{any::TypeId, cell::UnsafeCell};

use super::ComponentTrait;

/// A component.
///
/// This is a wrapper around a type that implements `ComponentTrait`.
///
/// This ignores Rust's borrow checker as it uses internal mutability.
///
/// Even though this goes against Rust's safety rules, it is the user's responsibility to ensure that
/// the rules are followed. Hopefully this will be fixed in the future.
pub struct Component {
    data: Box<UnsafeCell<dyn ComponentTrait>>,
}

unsafe impl Send for Component {}
unsafe impl Sync for Component {}

impl Component {
    /// Creates a new component from a type that implements `ComponentTrait`.
    #[must_use]
    pub fn new<T>(component: T) -> Self
    where
        T: ComponentTrait + 'static,
    {
        Self {
            data: Box::new(UnsafeCell::new(component)),
        }
    }

    /// Gets the inner component type.
    #[must_use]
    pub fn take<T: ComponentTrait + Sized>(self) -> T {
        unsafe {
            let ptr = self.data.get() as *const T;
            let t = ptr.read();
            std::mem::forget(self);
            t
        }
    }

    /// Gets a reference to the component as `T`.
    #[must_use]
    pub fn get<T: ComponentTrait>(&self) -> &T {
        unsafe { &*self.data.get().cast::<T>() }
    }

    /// Gets a mutable reference to the component as `T`.
    #[must_use]
    pub fn get_mut<T: ComponentTrait>(&self) -> &mut T {
        unsafe { &mut *self.data.get().cast::<T>() }
    }

    /// Gets a reference to the component as `dyn ComponentTrait`.
    #[must_use]
    pub fn get_trait(&self) -> &dyn ComponentTrait {
        unsafe { &*self.data.get() }
    }

    /// Gets a mutable reference to the component as `dyn ComponentTrait`.
    #[must_use]
    pub fn get_trait_mut(&self) -> &mut dyn ComponentTrait {
        unsafe { &mut *self.data.get() }
    }

    /// Gets the type id of the underlying component.
    #[must_use]
    pub fn type_id(&self) -> TypeId {
        self.get_trait().type_id()
    }

    /// Checks if the component is of the given type.
    #[must_use]
    pub fn is_type<T: ComponentTrait>(&self) -> bool {
        self.type_id() == TypeId::of::<T>()
    }

    /// Gets the type name of the underlying component.
    #[must_use]
    pub fn type_name(&self) -> &'static str {
        self.get_trait().dyn_type_name()
    }
}
