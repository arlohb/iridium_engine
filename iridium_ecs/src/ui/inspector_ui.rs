/// A trait implemented by something that can be drawn to the inspector UI.
pub trait InspectorUi {
    /// Draws the thing to the inspector UI.
    fn ui(&mut self, ui: &mut egui::Ui);
}
