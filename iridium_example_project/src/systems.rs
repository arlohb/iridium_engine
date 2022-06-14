use iridium_ecs::{*, systems::System};

pub struct CustomSystem;

impl System for CustomSystem {
    fn name(&self) -> &'static str { "CustomSystem" }

    fn component_type(&self) -> ComponentType {
        create_component_type! { struct CustomComponent {
            test: f64,
        }}
    }

    fn system(&self, entities: &Entities, _delta_time: f64) {
        for [custom_component]
        in entities.query(["CustomComponent"]) {
            println!("Custom component value: {}", custom_component.get::<f64>("test"));
        }
    }
}
