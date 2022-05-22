use iridium_ecs::*;
use iridium_ecs_macros::*;

#[derive(System)]
pub struct CustomSystem {
    pub activated: bool,
}

impl CustomSystem {
    fn run(&mut self, entities: &mut Entities, _delta_time: f64) {
        for entity in entities.query(vec!["CustomComponent"]).iter() {
            let custom_component = entity.get_component("CustomComponent").unwrap();
            println!("Custom component value: {}", custom_component.get::<f64>("value").unwrap());
        }
    }
}
