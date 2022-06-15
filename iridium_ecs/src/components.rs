use crate::ComponentTrait;

pub struct Transform {
    pub position: iridium_maths::Vec3,
    pub scale: iridium_maths::Vec3,
    pub rotation: f32,
}

impl ComponentTrait for Transform {
    fn type_name() -> &'static str { "Transform" }
    fn dyn_type_name(&self) -> &'static str { "Transform" }
}

pub struct Velocity {
    pub velocity: iridium_maths::Vec3,
}

impl ComponentTrait for Velocity {
    fn type_name() -> &'static str { "Velocity" }
    fn dyn_type_name(&self) -> &'static str { "Velocity" }
}
