pub struct EguiPanel {
    pub rpass: egui_latest_wgpu_backend::RenderPass,
    pub ui: Box<dyn super::PanelUi>,
}

impl EguiPanel {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        ui: impl super::PanelUi + 'static,
    ) -> EguiPanel {
        let rpass = egui_latest_wgpu_backend::RenderPass::new(
            device,
            format,
            1,
        );

        EguiPanel {
            rpass,
            ui: Box::new(ui),
        }
    }
}
