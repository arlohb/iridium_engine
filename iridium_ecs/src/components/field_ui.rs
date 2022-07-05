/// The attributes a component field has.
/// 
/// Map is attribute name -> attribute value.
pub struct ComponentFieldAttributes(pub hashbrown::HashMap<&'static str, &'static str>);

/// A trait implemented by component fields that aren't hidden.
pub trait ComponentFieldUi {
    /// Draws the ui for the component field.
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

impl<const N: usize> ComponentFieldUi for iridium_maths::VecN<N> {
    fn ui(&mut self, ui: &mut egui::Ui, attributes: ComponentFieldAttributes) {
        let mut values = self.data.to_vec();

        ui.columns(N, |ui| {
            for i in 0..N {
                let mut drag_value = egui::DragValue::new(&mut values[i]);

                if let Some(drag_speed) = attributes.0.get("drag_speed") {
                    drag_value = drag_value.speed(drag_speed.parse::<f32>().unwrap());
                }

                ui[i].add(drag_value);
            }
        });

        self.data.copy_from_slice(&values);
    }
}
