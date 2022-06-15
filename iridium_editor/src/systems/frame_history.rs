use std::{collections::VecDeque, time::SystemTime};

use iridium_ecs::{*, systems::System};
use iridium_ecs_macros::ComponentTrait;

pub struct Frame {
    pub time: SystemTime,
    pub delta_time: f64,
}

#[derive(ComponentTrait)]
pub struct FrameHistoryState {
    pub frames: VecDeque<Frame>,
    pub max_frames: usize,
    pub max_age: f64,
}

pub struct FrameHistorySystem;

impl System for FrameHistorySystem {
    fn name(&self) -> &'static str { "FrameHistorySystem" }

    fn component_type(&self) -> &'static str {
        "FrameHistoryState"
    }

    fn system(&self, entities: &Entities, delta_time: f64) {
        let mut component = entities.get::<FrameHistoryState>();
        let state = component.component::<FrameHistoryState>();

        state.frames.push_back(Frame {
            time: std::time::SystemTime::now(),
            delta_time,
        });
        if state.frames.len() > state.max_frames {
            state.frames.pop_front();
        }

        while let Some(frame) = state.frames.front() {
            if frame.time.elapsed().unwrap().as_millis() > state.max_age as u128 {
                state.frames.pop_front();
            } else {
                break;
            }
        }
    }
}

impl FrameHistorySystem {
    pub fn average_delta_time(state: &FrameHistoryState) -> f64 {
        let frames = &state.frames;
    
        frames
            .iter()
            .map(|frame| frame.delta_time)
            .sum::<f64>()
            / frames.len() as f64
    }
}
