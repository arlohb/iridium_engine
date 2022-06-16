pub struct ComponentFieldAttributes(pub hashbrown::HashMap<&'static str, &'static str>);

pub trait ComponentFieldUi {
    fn ui(&mut self, ui: &mut egui::Ui, attributes: ComponentFieldAttributes);
}

impl ComponentFieldUi for f32 {
    fn ui(&mut self, ui: &mut egui::Ui, _attributes: ComponentFieldAttributes) {
        ui.add(egui::DragValue::new(self));
    }
}

impl ComponentFieldUi for f64 {
    fn ui(&mut self, ui: &mut egui::Ui, _attributes: ComponentFieldAttributes) {
        ui.add(egui::DragValue::new(self));
    }
}

impl ComponentFieldUi for usize {
    fn ui(&mut self, ui: &mut egui::Ui, _attributes: ComponentFieldAttributes) {
        ui.add(egui::DragValue::new(self));
    }
}

impl ComponentFieldUi for String {
    fn ui(&mut self, ui: &mut egui::Ui, _attributes: ComponentFieldAttributes) {
        ui.text_edit_singleline(self);
    }
}

impl ComponentFieldUi for iridium_maths::Vec3 {
    fn ui(&mut self, ui: &mut egui::Ui, _attributes: ComponentFieldAttributes) {
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut self.x));
            ui.add(egui::DragValue::new(&mut self.y));
            ui.add(egui::DragValue::new(&mut self.z));
        });
    }
}
