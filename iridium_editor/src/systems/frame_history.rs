use std::{collections::VecDeque, time::SystemTime, sync::MutexGuard};

use iridium_ecs::{systems::System, Component, create_component_type};

pub struct Frame {
    pub time: SystemTime,
    pub delta_time: f64,
}

pub fn frame_history_system() -> System {
    System {
        name: "FrameHistorySystem",
        component_type: create_component_type! {
            struct FrameHistoryState {
                frames: VecDeque<Frame>,
                max_frames: usize,
                max_age: f64,
            }
        },
        system: |entities, delta_time| {
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

            println!("Fps average: {:.1}", 1000. / frame_history_average_delta_time(&state));
        },
    }
}

pub fn frame_history_average_delta_time(frame_history_component: &MutexGuard<Component>) -> f64 {
    let frames = frame_history_component.get::<VecDeque<Frame>>("frames");

    frames
        .iter()
        .map(|frame| frame.delta_time)
        .sum::<f64>()
        / frames.len() as f64
}
