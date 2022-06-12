use iridium_ecs::World;

use super::{PanelUi, UiState};

pub struct EngineUi {
    pub panels: Vec<Box<dyn PanelUi>>,
}

impl EngineUi {
    pub fn new() -> EngineUi {
        EngineUi {
            panels: vec![
                Box::new(super::panels::EntitiesList),
                Box::new(super::panels::ComponentsList),
            ],
        }
    }

    pub fn render(&mut self, context: &egui::Context, ui_state: &mut UiState, world: &mut World) {
        for panel in &mut self.panels {
            panel.render(context, ui_state, world);
        }
    }
}
