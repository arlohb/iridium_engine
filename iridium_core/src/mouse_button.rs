use strum_macros::{Display, EnumIter, EnumString};

/// Represents a mouse button.
#[derive(Display, EnumString, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle mouse button.
    Middle,
    /// Any other mouse button.
    Other(u16),
}

impl From<egui::PointerButton> for MouseButton {
    fn from(value: egui::PointerButton) -> Self {
        match value {
            egui::PointerButton::Primary => Self::Left,
            egui::PointerButton::Secondary => Self::Right,
            egui::PointerButton::Middle => Self::Middle,
            egui::PointerButton::Extra1 => Self::Other(3),
            egui::PointerButton::Extra2 => Self::Other(4),
        }
    }
}

impl From<winit::event::MouseButton> for MouseButton {
    fn from(value: winit::event::MouseButton) -> Self {
        match value {
            winit::event::MouseButton::Left => Self::Left,
            winit::event::MouseButton::Right => Self::Right,
            winit::event::MouseButton::Middle => Self::Middle,
            winit::event::MouseButton::Other(i) => Self::Other(i),
        }
    }
}
