use iridium_ecs_macros::Component;

#[derive(Component, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    /// If 3D, this is the z-axis.
    /// If 2D, this is the sorting layer.
    pub z: f64,
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
