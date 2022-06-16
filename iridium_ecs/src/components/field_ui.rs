pub struct ComponentFieldAttributes(pub hashbrown::HashMap<&'static str, &'static str>);

pub trait ComponentFieldUi {
    fn ui(&mut self, ui: &mut egui::Ui, attributes: ComponentFieldAttributes);
}

macro_rules! impl_component_field_ui_num {
    ($($ty:ty),*) => {
        $(
            impl ComponentFieldUi for $ty {
                fn ui(&mut self, ui: &mut egui::Ui, _attributes: ComponentFieldAttributes) {
                    ui.add(egui::DragValue::new(self));
                }
            }
        )*
    };
}

impl_component_field_ui_num!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

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
