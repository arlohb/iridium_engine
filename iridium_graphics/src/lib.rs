use iridium_ecs::*;
use iridium_ecs_macros::*;

#[derive(System)]
pub struct Renderer2DSystem {
    activated: bool,

    _data: f64,
}

impl Renderer2DSystem {
    pub fn new(activated: bool) -> Self {
        Self {
            activated,
            _data: 1.,
        }
    }

    fn run(&mut self, _entities: &mut Entities, _delta_time: f64) {
        
    }
}
