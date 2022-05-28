use std::{collections::VecDeque, time::SystemTime};

use iridium_ecs::Entities;
use iridium_ecs_macros::System;

pub struct Frame {
    pub time: SystemTime,
    pub delta_time: f64,
}

#[derive(System)]
pub struct FrameHistorySystem {
    activated: bool,

    frames: VecDeque<Frame>,
    max_frames: usize,
    max_age: f64,
}

impl FrameHistorySystem {
    pub fn new(activated: bool, max_frames: usize, max_age: f64) -> Self {
        Self {
            activated,
            frames: VecDeque::with_capacity(max_frames),
            max_frames,
            max_age,
        }
    }

    fn average_delta_time(&self) -> f64 {
        self.frames
            .iter()
            .map(|frame| frame.delta_time)
            .sum::<f64>()
            / self.frames.len() as f64
    }

    fn run(&mut self, _entities: &Entities, delta_time: f64) {
        self.frames.push_back(Frame {
            time: std::time::SystemTime::now(),
            delta_time,
        });
        if self.frames.len() > self.max_frames {
            self.frames.pop_front();
        }

        while let Some(frame) = self.frames.front() {
            if frame.time.elapsed().unwrap().as_millis() > self.max_age as u128 {
                self.frames.pop_front();
            } else {
                break;
            }
        }

        println!("Fps average: {:.1}", 1000. / self.average_delta_time());
    }
}