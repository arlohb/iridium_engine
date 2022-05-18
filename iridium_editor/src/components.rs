use iridium_ecs_macros::Component;

#[derive(Component, Default, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Default, Debug)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}
