use iridium_ecs::{*, systems::System};

use crate::components::Custom;

pub struct CustomState {
    pub test: f64,
}

pub struct CustomSystem;

impl System for CustomSystem {
    fn name(&self) -> &'static str { "CustomSystem" }

    fn system(&self, entities: &Entities, _delta_time: f64) {
        for [mut custom_component]
        in entities.query(["CustomComponent"]) {
            let custom_component = custom_component.component::<Custom>();
            println!("Custom component value: {}", custom_component.test);
        }
    }

    fn component_type(&self) -> &'static str {
        "CustomState"
    }
}
