use iridium_ecs::*;

pub struct Custom {
    pub test: f64,
}

impl ComponentTrait for Custom {
    fn type_name() -> &'static str { "Custom" }
    fn dyn_type_name(&self) -> &'static str { "Custom" }
}
