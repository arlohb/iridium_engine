pub struct ScreenRect {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl ScreenRect {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> ScreenRect {
        ScreenRect {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn egui_logical(&self, width: u32, height: u32, scale_factor: f32) -> egui::Rect {
        egui::Rect {
            min: egui::emath::pos2(
                self.min_x * (width as f32) / scale_factor,
                self.min_y * (height as f32) / scale_factor,
            ),
            max: egui::emath::pos2(
                self.max_x * (width as f32) / scale_factor,
                self.max_y * (height as f32) / scale_factor,
            ),
        }
    }
}

impl std::fmt::Debug for ScreenRect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[[{:.1} {:.1}] - [{:.1} {:.1}]]", self.min_x, self.min_y, self.max_x, self.max_y)
    }
}
