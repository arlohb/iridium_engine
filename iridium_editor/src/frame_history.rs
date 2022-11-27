use std::{collections::VecDeque, time::SystemTime};

use iridium_assets::Assets;
use iridium_ecs::{
    storage::{ComponentStorage, StoredComponent, StoredComponentField},
    systems::System,
    Component, Entities,
};
use iridium_ecs_macros::{ComponentTrait, InspectorUi};
use iridium_map_utils::fast_map;

/// Data about a single frame.
pub struct Frame {
    /// The start time of the frame.
    pub time: SystemTime,
    /// How long the frame lasted in ms.
    pub delta_time: f64,
}

/// The state of `FrameHistorySystem`.
#[derive(ComponentTrait, InspectorUi)]
pub struct FrameHistoryState {
    /// The queue of previous frames.
    #[hidden]
    pub frames: VecDeque<Frame>,
    /// The maximum number of frames to store.
    pub max_frames: usize,
    /// The maximum age of frames to store.
    pub max_age: f64,
}

impl FrameHistoryState {
    /// Calculates the average frame time in ms.
    #[must_use]
    pub fn average_delta_time(&self) -> f64 {
        self.frames
            .iter()
            .map(|frame| frame.delta_time)
            .sum::<f64>()
            / self.frames.len() as f64
    }

    /// Calculates the average frame rate in frames per second.
    #[must_use]
    pub fn average_fps(&self) -> f64 {
        1000. / self.average_delta_time()
    }
}

impl ComponentStorage for FrameHistoryState {
    fn from_stored(mut stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        let max_frames = stored.get("max_frames")?.parse().ok()?;

        Some(Self {
            frames: VecDeque::with_capacity(max_frames),
            max_frames,
            max_age: stored.get("max_age")?.parse().ok()?,
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "FrameHistoryState".to_string(),
            fields: fast_map! {
                "max_frames" => StoredComponentField::new(self.max_frames.to_string(), false),
                "max_age" => StoredComponentField::new(self.max_age.to_string(), false),
            },
        }
    }
}

/// A system to store data about previous frames.
pub struct FrameHistorySystem;

impl System for FrameHistorySystem {
    fn name(&self) -> &'static str {
        "FrameHistorySystem"
    }

    fn state_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<FrameHistoryState>()
    }

    fn default_state(&self) -> Component {
        Component::new(FrameHistoryState {
            frames: VecDeque::with_capacity(500_000),
            max_frames: 500_000,
            max_age: 5000.,
        })
    }

    fn system(&self, state: &Component, _entities: &Entities, _assets: &Assets, delta_time: f64) {
        let state = state.get_mut::<FrameHistoryState>();

        state.frames.push_back(Frame {
            time: std::time::SystemTime::now(),
            delta_time,
        });
        if state.frames.len() > state.max_frames {
            state.frames.pop_front();
        }

        while let Some(frame) = state.frames.front() {
            if frame
                .time
                .elapsed()
                .expect("Time went backwards")
                .as_millis()
                > state.max_age as u128
            {
                state.frames.pop_front();
            } else {
                break;
            }
        }
    }
}
