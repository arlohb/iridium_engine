pub struct UiState {
    pub viewport_rect: super::ScreenRect,
    pub screen_size: (u32, u32),
    pub scale_factor: f32,
    pub selected_entity: Option<u128>,
}

impl UiState {
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
