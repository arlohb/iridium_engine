use iridium_assets::Asset;

use super::InspectorUiFieldAttributes;

/// A trait implemented by component fields that aren't hidden.
pub trait InspectorUiField {
    /// Draws the ui for the component field.
    fn ui(&mut self, ui: &mut egui::Ui, attributes: InspectorUiFieldAttributes);
}

macro_rules! impl_inspector_ui_field_num {
    ($($ty:ty),*) => {
        $(
            impl InspectorUiField for $ty {
                fn ui(&mut self, ui: &mut egui::Ui, attributes: InspectorUiFieldAttributes) {
                    let mut drag_value = egui::DragValue::new(self);

                    if let Some(drag_speed) = attributes.get::<f32>("drag_speed") {
                        drag_value = drag_value.speed(drag_speed);
                    }

                    ui.add(drag_value);
                }
            }
        )*
    };
}

impl_inspector_ui_field_num!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

impl InspectorUiField for String {
    fn ui(&mut self, ui: &mut egui::Ui, _attributes: InspectorUiFieldAttributes) {
        ui.text_edit_singleline(self);
    }
}

impl InspectorUiField for bool {
    fn ui(&mut self, ui: &mut egui::Ui, _attributes: InspectorUiFieldAttributes) {
        ui.checkbox(self, "");
    }
}

impl<const N: usize> InspectorUiField for iridium_maths::VecN<N> {
    fn ui(&mut self, ui: &mut egui::Ui, attributes: InspectorUiFieldAttributes) {
        let mut values = self.data.to_vec();

        ui.columns(N, |ui| {
            for i in 0..N {
                let mut drag_value = egui::DragValue::new(&mut values[i]);

                if let Some(drag_speed) = attributes.get::<f32>("drag_speed") {
                    drag_value = drag_value.speed(drag_speed);
                }

                ui[i].add(drag_value);
            }
        });

        self.data.copy_from_slice(&values);
    }
}

impl<T: Send + Sync> InspectorUiField for Asset<T> {
    fn ui(&mut self, ui: &mut egui::Ui, _attributes: InspectorUiFieldAttributes) {
        // In the future, this will be editable.
        // For now just show a clone.
        let mut id = self.id.clone();
        ui.text_edit_singleline(&mut id);
    }
}

impl<T: InspectorUiField, const N: usize> InspectorUiField for [T; N] {
    fn ui(&mut self, ui: &mut egui::Ui, _attributes: InspectorUiFieldAttributes) {
        ui.columns(N, |ui| {
            for i in 0..N {
                self[i].ui(&mut ui[i], InspectorUiFieldAttributes::default());
            }
        });
    }
}
