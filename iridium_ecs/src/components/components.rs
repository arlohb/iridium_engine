use iridium_ecs_macros::ComponentTrait;

use crate::ComponentFieldUi;

use crate as iridium_ecs;

#[derive(ComponentTrait)]
pub struct Name {
    pub name: String,
}

#[derive(ComponentTrait)]
pub struct Transform {
    #[drag_speed(0.05)]
    pub position: iridium_maths::Vec3,
    #[drag_speed(0.05)]
    pub scale: iridium_maths::Vec3,
    #[drag_speed(0.05)]
    pub rotation: f32,
}

#[derive(ComponentTrait)]
pub struct Velocity {
    #[drag_speed(0.0001)]
    pub velocity: iridium_maths::Vec3,
}
