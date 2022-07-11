use image::GenericImageView;
use iridium_assets::Assets;

use crate::ui::PanelUi;

fn load_texture(context: &egui::Context, path: &str) -> ((usize, usize), egui::TextureHandle) {
    let reader = image::io::Reader::open(path).expect("Failed to open texture");
    let dynamic_image = reader.decode().expect("Failed to decode texture");
    let size = dynamic_image.dimensions();
    let image_rgba = dynamic_image.to_rgba8();

    let texture = context.load_texture(
        path.split('/').last().unwrap_or(path),
        egui::ColorImage::from_rgba_unmultiplied([size.0 as usize, size.1 as usize], &image_rgba),
    );

    ((size.0 as usize, size.1 as usize), texture)
}

#[derive(PartialEq)]
pub enum CurrentTab {
    Assets,
    Profiler,
}

pub struct BottomPanel {
    texture: Option<egui::TextureHandle>,
    current_tab: CurrentTab,
}

impl BottomPanel {
    pub const fn new() -> Self {
        Self {
            texture: None,
            current_tab: CurrentTab::Profiler,
        }
    }
}

impl PanelUi for BottomPanel {
    fn name(&self) -> &'static str {
        "BottomPanel"
    }

    fn render(
        &mut self,
        context: &egui::Context,
        ui_state: &mut crate::ui::UiState,
        _world: &mut iridium_ecs::World,
        _assets: &Assets,
    ) {
        egui::TopBottomPanel::bottom("asset_browser")
            .default_height(ui_state.screen_size.1 as f32 * ui_state.scale_factor * 0.3)
            .resizable(true)
            .show(context, |ui| {
                let min_y_logical = ui.max_rect().min.y;
                let min_y_physical = min_y_logical * ui_state.scale_factor;
                let min_y_screen = min_y_physical / ui_state.screen_size.1 as f32;
                ui_state.viewport_rect.max_y = min_y_screen;

                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.current_tab, CurrentTab::Assets, "Assets");
                    ui.selectable_value(&mut self.current_tab, CurrentTab::Profiler, "Profiler");
                });

                ui.separator();

                match self.current_tab {
                    CurrentTab::Assets => {
                        let texture = self.texture.get_or_insert_with(|| {
                            load_texture(ui.ctx(), "iridium_editor/assets/FoodSprites/Food.png").1
                        });

                        ui.label("This is the asset browser");

                        egui::ScrollArea::new([false, true])
                            .auto_shrink([false, false])
                            .max_width(f32::INFINITY)
                            .always_show_scroll(true)
                            .show(ui, |ui| {
                                ui.image(texture, (200., 200.));
                            });
                    }
                    CurrentTab::Profiler => {
                        puffin_egui::profiler_ui(ui);
                    }
                }
            });
    }
}
