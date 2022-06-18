#![allow(clippy::mut_from_ref)]

use std::{cell::UnsafeCell, any::{Any, TypeId}};

pub struct ComponentInfo {
    pub type_name: &'static str,
}

impl ComponentInfo {
    pub fn new<T>() -> Self
    where T: ComponentTrait {
        Self {
            type_name: T::type_name(),
        }
    }
}

pub trait ComponentFactory: ComponentTrait + Sized {
    fn create() -> Component;
}

pub trait ComponentTrait: 'static + Send + Sync + Any {
    fn type_name() -> &'static str where Self: Sized;
    fn dyn_type_name(&self) -> &'static str;
    fn field_types(&self) -> Vec<(&'static str, &'static str)>;
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub struct Component {
    data: Box<UnsafeCell<dyn ComponentTrait>>,
}

unsafe impl Send for Component {}
unsafe impl Sync for Component {}

impl Component {
    pub fn new<T>(component: T) -> Self
    where T: ComponentTrait + 'static {
        Component {
            data: Box::new(UnsafeCell::new(component)),
        }
    }

    pub fn get<T: ComponentTrait>(&self) -> &T {
        unsafe {
            &*(self.data.get() as *const _ as *const T)
        }
    }

    pub fn get_mut<T: ComponentTrait>(&self) -> &mut T {
        unsafe {
            &mut *(self.data.get() as *mut _ as *mut T)
        }
    }

    pub fn get_trait(&self) -> &dyn ComponentTrait {
        unsafe {
            &*self.data.get()
        }
    }

    pub fn get_trait_mut(&self) -> &mut dyn ComponentTrait {
        unsafe {
            &mut *self.data.get()
        }
    }

    pub fn type_id(&self) -> TypeId {
        self.get_trait().type_id()
    }

    pub fn is_type<T: ComponentTrait>(&self) -> bool {
        self.type_id() == TypeId::of::<T>()
    }

    pub fn type_name(&self) -> &'static str {
        self.get_trait().dyn_type_name()
    }
}
