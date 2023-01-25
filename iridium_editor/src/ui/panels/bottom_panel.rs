use image::GenericImageView;
use iridium_assets::Assets;
use iridium_core::{LogState, LogType};

use crate::ui::PanelUi;

fn load_texture(context: &egui::Context, path: &str) -> ((usize, usize), egui::TextureHandle) {
    let reader = image::io::Reader::open(path).expect("Failed to open texture");
    let dynamic_image = reader.decode().expect("Failed to decode texture");
    let size = dynamic_image.dimensions();
    let image_rgba = dynamic_image.to_rgba8();

    let texture = context.load_texture(
        path.split('/').last().unwrap_or(path),
        egui::ColorImage::from_rgba_unmultiplied([size.0 as usize, size.1 as usize], &image_rgba),
        egui::TextureFilter::Nearest,
    );

    ((size.0 as usize, size.1 as usize), texture)
}

#[derive(PartialEq, Eq)]
pub enum CurrentTab {
    Assets,
    Logs,
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
            current_tab: CurrentTab::Assets,
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
        world: &mut iridium_ecs::World,
        assets: &Assets,
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
                    ui.selectable_value(&mut self.current_tab, CurrentTab::Logs, "Logs");
                    ui.selectable_value(&mut self.current_tab, CurrentTab::Profiler, "Profiler");
                });

                ui.separator();

                match self.current_tab {
                    CurrentTab::Assets => {
                        let icon_size = 40.;

                        let icons_in_row =
                            (ui.available_width() / (icon_size + 3.5)).trunc() as usize - 1;

                        let all_assets = assets.get_all();

                        let texture = &*self.texture.get_or_insert_with(|| {
                            load_texture(
                                ui.ctx(),
                                "iridium_example_project/assets/FoodSprites/Food.png",
                            )
                            .1
                        });

                        egui::ScrollArea::new([false, true])
                            .auto_shrink([false, false])
                            .max_width(f32::INFINITY)
                            .always_show_scroll(true)
                            .show(ui, |ui| {
                                egui::Grid::new("Asset grid").show(ui, |ui| {
                                    for (index, (id, _asset)) in all_assets.into_iter().enumerate()
                                    {
                                        if index % icons_in_row == 0 {
                                            ui.end_row();
                                        }

                                        ui.vertical(|ui| {
                                            ui.image(texture, (icon_size, icon_size));
                                            ui.label(id);
                                        });
                                    }
                                });
                            });
                    }
                    CurrentTab::Logs => {
                        egui::ScrollArea::new([false, true])
                            .auto_shrink([false, false])
                            .max_width(f32::INFINITY)
                            .always_show_scroll(true)
                            .show(ui, |ui| {
                                let log = world.entities.get::<LogState>();

                                for entry in log.entries() {
                                    match entry.log_type {
                                        LogType::Info => ui.label(&entry.message),
                                        LogType::Warning => ui.label(
                                            egui::RichText::new(&entry.message)
                                                .color(egui::Color32::YELLOW),
                                        ),
                                        LogType::Error => ui.label(
                                            egui::RichText::new(&entry.message)
                                                .color(egui::Color32::RED),
                                        ),
                                    };
                                }
                            });
                    }
                    CurrentTab::Profiler => {
                        puffin_egui::profiler_ui(ui);
                    }
                }
            });
    }
}
