use iridium_ecs::World;
use winit::window::Window;

use super::{PanelUi, UiState};

/// The rendering state of the editor UI.
pub struct EguiState {
    /// The egui context.
    pub context: egui::Context,
    /// The egui backend state.
    pub rpass: egui_latest_wgpu_backend::RenderPass,
    /// The egui winit state.
    pub winit: egui_winit::State,
    /// The UI panels.
    pub panels: Vec<Box<dyn PanelUi>>,
}

impl EguiState {
    /// Creates a new egui state.
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        window: &Window,
    ) -> EguiState {
        // Create the egui context.
        let context = egui::Context::default();

        // Create the egui backend state.
        let rpass = egui_latest_wgpu_backend::RenderPass::new(
            device,
            format,
            1,
        );

        // Create the winit state.
        let winit = egui_winit::State::new(4096, window);

        // Create the UI panels.
        let panels: Vec<Box<dyn PanelUi>> = vec![
            Box::new(super::panels::TopPanel),
            Box::new(super::panels::EntitiesList::new()),
            Box::new(super::panels::ComponentsList),
            Box::new(super::panels::AssetBrowser::new()),
        ];

        EguiState {
            context,
            rpass,
            winit,
            panels,
        }
    }

    /// Renders all the panels.
    pub fn render_panels(&mut self, ui_state: &mut UiState, world: &mut World) {
        for panel in &mut self.panels {
            panel.render(&self.context, ui_state, world);
        }
    }
}
