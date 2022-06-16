use iridium_ecs_macros::ComponentTrait;
use iridium_ecs::ComponentFieldUi;

#[derive(ComponentTrait)]
pub struct Custom {
    pub test: f64,
}
