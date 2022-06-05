pub trait PanelUi {
    fn render(&mut self, context: &egui::Context);
}

impl PanelUi for egui_demo_lib::DemoWindows {
    fn render(&mut self, context: &egui::Context) {
        self.ui(context);
    }
}
