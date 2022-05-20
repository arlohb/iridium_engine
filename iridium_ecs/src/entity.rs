use super::*;

#[derive(Debug)]
pub struct Entity {
    pub id: u128,
    pub components: Vec<Box<dyn Component>>,
}

impl Entity {
    pub fn new(components: Vec<Box<dyn Component>>) -> Entity {
        Entity {
            id: 0,
            components,
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    pub fn get_component<T>(&self) -> Option<&mut T>
        where T: Component
    {
        self.components.iter().find_map(|comp_box| {
            if comp_box.dyn_get_type() == T::get_type() {
                Some({
                    let c_value = &**comp_box;
                    // get the raw pointer to the component
                    let comp_ptr = c_value as *const dyn Component;
                    // cast the raw pointer to the type of the component
                    let t_ptr = comp_ptr as *mut T;
                    // convert the pointer to a reference
                    // this is safe because we know that the component is of type T
                    unsafe { &mut *t_ptr }
                })
            } else {
                None
            }
        })
    }
}
