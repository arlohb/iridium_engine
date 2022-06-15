// Stores the editor UI state.
pub struct UiState {
    /// The screen rect the viewport covers.
    pub viewport_rect: super::ScreenRect,
    /// The screen size of the editor in physical pixels.
    pub screen_size: (u32, u32),
    /// The scale factor of the egui UI.
    pub scale_factor: f32,
    /// The id of the currently selected entity.
    pub selected_entity: Option<u128>,
}

impl UiState {
    /// Creates a new UI state.
    pub fn new(
        viewport_rect: super::ScreenRect,
        screen_size: (u32, u32),
        scale_factor: f32,
    ) -> Self {
        UiState {
            viewport_rect,
            screen_size,
            scale_factor,
            selected_entity: None,
        }
    }
}
