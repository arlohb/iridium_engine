use iridium_ecs::*;
use iridium_ecs_macros::*;

#[derive(System)]
pub struct Renderer2DSystem {
    activated: bool,

    data: f64,
}

impl Renderer2DSystem {
    pub fn new(activated: bool) -> Self {
        Self {
            activated,
            data: 1.,
        }
    }

    fn run(&mut self, entities: &mut Entities) {
        
    }
}
