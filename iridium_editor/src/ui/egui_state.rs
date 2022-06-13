use iridium_ecs::World;
use winit::window::Window;

use super::{PanelUi, UiState};

pub struct EguiState {
    pub context: egui::Context,
    pub rpass: egui_latest_wgpu_backend::RenderPass,
    pub winit: egui_winit::State,
    pub panels: Vec<Box<dyn PanelUi>>,
}

impl EguiState {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        window: &Window,
    ) -> EguiState {
        let context = egui::Context::default();
        let rpass = egui_latest_wgpu_backend::RenderPass::new(
            device,
            format,
            1,
        );
        let winit = egui_winit::State::new(4096, window);
        let panels: Vec<Box<dyn PanelUi>> = vec![
            Box::new(super::panels::EntitiesList),
            Box::new(super::panels::ComponentsList),
        ];

        EguiState {
            context,
            rpass,
            winit,
            panels,
        }
    }

    pub fn render_panels(&mut self, ui_state: &mut UiState, world: &mut World) {
        for panel in &mut self.panels {
            panel.render(&self.context, ui_state, world);
        }
    }
}
