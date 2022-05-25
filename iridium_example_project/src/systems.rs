use iridium_ecs::*;
use iridium_ecs_macros::*;

#[derive(System)]
pub struct CustomSystem {
    pub activated: bool,
}

impl CustomSystem {
    fn run(&mut self, entities: &Entities, _delta_time: f64) {
        for [custom_component] in entities.query(["CustomComponent"]) {
            println!("Custom component value: {}", custom_component.get::<f64>("test"));
        }
    }
}
