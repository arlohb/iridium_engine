use crate::play_state::PlayState;

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
    /// The current play state.
    play_state: PlayState,
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
            play_state: PlayState::Stop,
        }
    }

    /// Gets the play state.
    pub fn play_state(&self) -> PlayState {
        self.play_state
    }

    /// Sets the plat state to `PlayState::Play`.
    pub fn play(&mut self) {
        self.play_state = PlayState::Play;
    }

    /// Sets the plat state to `PlayState::Pause`.
    pub fn pause(&mut self) {
        self.play_state = PlayState::Pause;
    }

    /// Sets the plat state to `PlayState::Stop`.
    pub fn stop(&mut self) {
        // Eventually this will do quite a bit more, but that comes later.
        self.play_state = PlayState::Stop;
    }
}
