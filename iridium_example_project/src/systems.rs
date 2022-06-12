use iridium_ecs::{systems::System, create_component_type};

pub fn custom_system() -> System {
    System {
        name: "CustomSystem",
        component_type: create_component_type! {struct CustomComponent {
            test: f64,
        }},
        system: |entities, _delta_time| {
            for [custom_component]
            in entities.query(["CustomComponent"]) {
                println!("Custom component value: {}", custom_component.get::<f64>("test"));
            }
        },
    }
}
