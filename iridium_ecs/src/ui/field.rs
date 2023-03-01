use egui::Widget;
use iridium_assets::{Asset, AssetBox};

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

// This implementation is mainly just for reference,
// as most things that use assets need to regenerate
// some data when an asset changes, so will need to
// impl `InspectorUi` themselves anyway.
impl<T: Asset> InspectorUiField for AssetBox<T> {
    fn ui(&mut self, ui: &mut egui::Ui, _attributes: InspectorUiFieldAttributes) {
        // Copy the id
        let mut id = self.id().to_owned();

        // Allow user to edit
        egui::TextEdit::singleline(&mut id)
            .desired_width(f32::INFINITY)
            .ui(ui);

        // If the id has changed
        if self.id() != id {
            // Update it
            self.change_id(id);
        }
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
