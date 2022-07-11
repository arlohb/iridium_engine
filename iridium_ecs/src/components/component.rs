#![allow(clippy::mut_from_ref)]

use std::{
    any::{Any, TypeId},
    cell::UnsafeCell,
};

use iridium_assets::Assets;

use crate::storage::{ComponentStorage, StoredComponent};

/// Information about a component type when it is registered.
///
/// Right now this is just the type name, in the future this may include field types.
pub struct ComponentInfo {
    /// The name of the component type.
    pub type_name: &'static str,
    /// Creates a component from within the UI.
    ///
    /// Not all components implement this.
    pub default: Option<fn() -> Component>,
    /// Tries to create a component from a stored component.
    pub from_stored: fn(StoredComponent, &Assets) -> Option<Component>,
}

impl ComponentInfo {
    /// Creates a new component info from a component type.
    pub fn new<T>() -> Self
    where
        T: ComponentTrait,
    {
        Self {
            type_name: T::type_name(),
            default: None,
            from_stored: T::from_stored_component,
        }
    }

    /// Creates a new component info from a component type.
    ///
    /// Also adds the default fn.
    pub fn new_with_default<T>() -> Self
    where
        T: ComponentTrait + ComponentDefault,
    {
        Self {
            type_name: T::type_name(),
            default: Some(T::create),
            from_stored: T::from_stored_component,
        }
    }
}

/// A trait implemented by components that can be created from the UI without any inputs.
pub trait ComponentDefault: ComponentTrait + Sized {
    /// Creates a new component from the default values.
    ///
    /// This is returned as a Component, not Self.
    fn create() -> Component;
}

/// A trait implemented by components.
pub trait ComponentTrait: 'static + Send + Sync + Any + ComponentStorage {
    /// The name of the component type.
    ///
    /// Called on the type.
    fn type_name() -> &'static str
    where
        Self: Sized;
    /// The name of the component type.
    ///
    /// Called on an instance of the type.
    fn dyn_type_name(&self) -> &'static str;
    /// A vec of the field name and the type.
    fn field_types(&self) -> Vec<(&'static str, &'static str)>;
    /// Draws the component field to the UI.
    fn ui(&mut self, ui: &mut egui::Ui);
}

/// A component.
///
/// This is a wrapper around a type that implements ComponentTrait.
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
    /// Creates a new component from a type that implements ComponentTrait.
    pub fn new<T>(component: T) -> Self
    where
        T: ComponentTrait + 'static,
    {
        Component {
            data: Box::new(UnsafeCell::new(component)),
        }
    }

    /// Gets the inner component type.
    pub fn take<T: ComponentTrait + Sized>(self) -> T {
        unsafe {
            let ptr = self.data.get() as *const _ as *const T;
            let t = ptr.read();
            std::mem::forget(self);
            t
        }
    }

    /// Gets a reference to the component as `T`.
    pub fn get<T: ComponentTrait>(&self) -> &T {
        unsafe { &*(self.data.get() as *const _ as *const T) }
    }

    /// Gets a mutable reference to the component as `T`.
    pub fn get_mut<T: ComponentTrait>(&self) -> &mut T {
        unsafe { &mut *(self.data.get() as *mut _ as *mut T) }
    }

    /// Gets a reference to the component as `dyn ComponentTrait`.
    pub fn get_trait(&self) -> &dyn ComponentTrait {
        unsafe { &*self.data.get() }
    }

    /// Gets a mutable reference to the component as `dyn ComponentTrait`.
    pub fn get_trait_mut(&self) -> &mut dyn ComponentTrait {
        unsafe { &mut *self.data.get() }
    }

    /// Gets the type id of the underlying component.
    pub fn type_id(&self) -> TypeId {
        self.get_trait().type_id()
    }

    /// Checks if the component is of the given type.
    pub fn is_type<T: ComponentTrait>(&self) -> bool {
        self.type_id() == TypeId::of::<T>()
    }

    /// Gets the type name of the underlying component.
    pub fn type_name(&self) -> &'static str {
        self.get_trait().dyn_type_name()
    }
}
