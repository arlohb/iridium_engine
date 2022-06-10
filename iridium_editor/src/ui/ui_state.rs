pub struct UiState {
    pub selected_entity: Option<u128>,
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            selected_entity: None,
        }
    }
}
