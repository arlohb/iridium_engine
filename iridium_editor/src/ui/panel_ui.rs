use iridium_ecs::World;

pub trait PanelUi {
    fn render(&mut self, context: &egui::Context, world: &mut World);
}

impl PanelUi for egui_demo_lib::DemoWindows {
    fn render(&mut self, context: &egui::Context, _: &mut World) {
        self.ui(context);
    }
}
