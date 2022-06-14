use std::{collections::VecDeque, time::SystemTime, sync::MutexGuard};

use iridium_ecs::{*, systems::System};

pub struct Frame {
    pub time: SystemTime,
    pub delta_time: f64,
}

pub struct FrameHistorySystem;

impl System for FrameHistorySystem {
    fn name(&self) -> &'static str { "FrameHistorySystem" }

    fn component_type(&self) -> ComponentType {
        create_component_type! { struct FrameHistoryState {
            frames: VecDeque<Frame>,
            max_frames: usize,
            max_age: f64,
        }}
    }

    fn system(&self, entities: &Entities, delta_time: f64) {
        let mut state = entities.get("FrameHistoryState");
        let max_frames = *state.get::<usize>("max_frames");
        let max_age = *state.get::<f64>("max_age");
        let frames = state.get_mut::<VecDeque<Frame>>("frames");

        frames.push_back(Frame {
            time: std::time::SystemTime::now(),
            delta_time,
        });
        if frames.len() > max_frames {
            frames.pop_front();
        }

        while let Some(frame) = frames.front() {
            if frame.time.elapsed().unwrap().as_millis() > max_age as u128 {
                frames.pop_front();
            } else {
                break;
            }
        }
    }
}

impl FrameHistorySystem {
    pub fn average_delta_time(frame_history_component: &MutexGuard<Component>) -> f64 {
        let frames = frame_history_component.get::<VecDeque<Frame>>("frames");
    
        frames
            .iter()
            .map(|frame| frame.delta_time)
            .sum::<f64>()
            / frames.len() as f64
    }
}
