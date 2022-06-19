pub struct ComponentFieldAttributes(pub hashbrown::HashMap<&'static str, &'static str>);

pub trait ComponentFieldUi {
    fn ui(&mut self, ui: &mut egui::Ui, attributes: ComponentFieldAttributes);
}

macro_rules! impl_component_field_ui_num {
    ($($ty:ty),*) => {
        $(
            impl ComponentFieldUi for $ty {
                fn ui(&mut self, ui: &mut egui::Ui, attributes: ComponentFieldAttributes) {
                    let mut drag_value = egui::DragValue::new(self);

                    if let Some(drag_speed) = attributes.0.get("drag_speed") {
                        drag_value = drag_value.speed(drag_speed.parse::<f32>().unwrap());
                    }

                    ui.add(drag_value);
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
    fn ui(&mut self, ui: &mut egui::Ui, attributes: ComponentFieldAttributes) {
        ui.horizontal(|ui| {
            let mut x_drag = egui::DragValue::new(&mut self.x);
            let mut y_drag = egui::DragValue::new(&mut self.y);
            let mut z_drag = egui::DragValue::new(&mut self.z);

            if let Some(drag_speed) = attributes.0.get("drag_speed") {
                x_drag = x_drag.speed(drag_speed.parse::<f32>().unwrap());
                y_drag = y_drag.speed(drag_speed.parse::<f32>().unwrap());
                z_drag = z_drag.speed(drag_speed.parse::<f32>().unwrap());
            }

            ui.columns(3, |ui| {
                ui[0].add(x_drag);
                ui[1].add(y_drag);
                ui[2].add(z_drag);
            });
        });
    }
}
