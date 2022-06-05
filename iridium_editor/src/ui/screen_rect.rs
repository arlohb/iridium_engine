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

    pub fn rect_physical(&self, width: u32, height: u32) -> egui::Rect {
        egui::Rect {
            min: egui::emath::pos2(
                self.min_x * width as f32,
                self.min_y * height as f32,
            ),
            max: egui::emath::pos2(
                self.max_x * width as f32,
                self.max_y * height as f32,
            ),
        }
    }

    pub fn rect_logical(&self, width: u32, height: u32, scale_factor: f32) -> egui::Rect {
        egui::Rect {
            min: egui::emath::pos2(
                self.min_x * width as f32 / scale_factor,
                self.min_y * height as f32 / scale_factor,
            ),
            max: egui::emath::pos2(
                self.max_x * width as f32 / scale_factor,
                self.max_y * height as f32 / scale_factor,
            ),
        }
    }
}
