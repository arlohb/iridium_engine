use iridium_ecs::{*, systems::System};
use iridium_ecs_macros::ComponentTrait;

use crate::components::Custom;

#[derive(ComponentTrait)]
pub struct CustomState {
    pub test: f64,
}

pub struct CustomSystem;

impl System for CustomSystem {
    fn name(&self) -> &'static str { "CustomSystem" }

    fn system(&self, entities: &Entities, _delta_time: f64) {
        for [custom_component]
        in entities.query(["CustomComponent"]) {
            let custom_component = custom_component.get::<Custom>();
            println!("Custom component value: {}", custom_component.test);
        }
    }

    fn component_type(&self) -> &'static str {
        "CustomState"
    }
}
