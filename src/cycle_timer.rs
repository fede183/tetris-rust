use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct CycleTimer {
    pub timer: Timer,
}

impl CycleTimer {
    pub fn new() -> Self {
        Self { timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating) }
    }
}

impl Default for CycleTimer {
    fn default() -> Self {
        Self::new()
    }
}
