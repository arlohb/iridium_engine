use winit::window::Window;

use super::EngineUi;

pub struct EguiState {
    pub context: egui::Context,
    pub rpass: egui_latest_wgpu_backend::RenderPass,
    pub winit: egui_winit::State,
    pub engine_ui: EngineUi,
}

impl EguiState {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        window: &Window,
        engine_ui: EngineUi,
    ) -> EguiState {
        let context = egui::Context::default();
        let winit = egui_winit::State::new(4096, window);
        let rpass = egui_latest_wgpu_backend::RenderPass::new(
            device,
            format,
            1,
        );

        EguiState {
            context,
            rpass,
            winit,
            engine_ui,
        }
    }
}
