use winit::window::Window;

pub struct EguiState {
    pub context: egui::Context,
    pub winit: egui_winit::State,
    pub scale_factor: f32,
}

impl EguiState {
    pub fn new(window: &Window, scale_factor: f32) -> EguiState {
        let winit = egui_winit::State::new(4096, window);
        let context = egui::Context::default();

        EguiState {
            context,
            winit,
            scale_factor,
        }
    }
}
