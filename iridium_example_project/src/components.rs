use iridium_ecs_macros::ComponentTrait;

#[derive(ComponentTrait)]
pub struct Custom {
    pub test: f64,
}
