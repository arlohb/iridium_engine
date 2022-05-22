use iridium_ecs::*;

pub fn component_types() -> Vec<ComponentType> {
    vec![
        ComponentType {
            name: "Custom".to_string(),
            values: fast_map! {
                "test" => "f64"
            },
        },
    ]
}
