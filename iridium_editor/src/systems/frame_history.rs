use std::collections::VecDeque;

use iridium_ecs::Entities;
use iridium_ecs_macros::System;

#[derive(System)]
pub struct FrameHistorySystem {
    activated: bool,

    delta_times: VecDeque<f64>,
    max_frames: usize,
}

impl FrameHistorySystem {
    pub fn new(activated: bool, max_frames: usize) -> Self {
        Self {
            activated,
            delta_times: VecDeque::with_capacity(max_frames),
            max_frames,
        }
    }

    fn run(&mut self, _entities: &Entities, delta_time: f64) {
        self.delta_times.push_back(delta_time);
        if self.delta_times.len() > self.max_frames {
            self.delta_times.pop_front();
        }

        let total = self.delta_times.iter().sum::<f64>();
        let average = total / self.delta_times.len() as f64;

        println!("Fps average: {:.1}", 1000. / average);
    }
}