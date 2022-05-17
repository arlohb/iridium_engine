use super::Component;

#[derive(Default, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Component for Position {
    fn get_type(&self) -> &'static str { "Position" }
}

#[derive(Default, Debug)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

impl Component for Velocity {
    fn get_type(&self) -> &'static str { "Velocity" }
}
