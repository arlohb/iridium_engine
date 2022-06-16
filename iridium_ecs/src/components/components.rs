use iridium_ecs_macros::ComponentTrait;

use crate::ComponentFieldUi;

use crate as iridium_ecs;

#[derive(ComponentTrait)]
pub struct Name {
    pub name: String,
}

#[derive(ComponentTrait)]
pub struct Transform {
    pub position: iridium_maths::Vec3,
    pub scale: iridium_maths::Vec3,
    pub rotation: f32,
}

#[derive(ComponentTrait)]
pub struct Velocity {
    pub velocity: iridium_maths::Vec3,
}
