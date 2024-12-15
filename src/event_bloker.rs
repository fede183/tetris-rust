use std::time::Duration;
use bevy::time::{Timer, TimerMode};
use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct EventBlocker {
    pub timer: Timer,
}


impl EventBlocker {
    pub fn new() -> Self {
        Self { timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Once) }
    }
}

// We need to implement Default so that our timer can be initialized as
// a resource when we call `init_resource`
impl Default for EventBlocker {
    fn default() -> Self {
        Self::new()
    }
}
