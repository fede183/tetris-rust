use bevy::time::{Timer, TimerMode};
use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct MatchTime(pub Timer);

impl MatchTime {
    pub fn new() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Once))
    }
}

// We need to implement Default so that our timer can be initialized as
// a resource when we call `init_resource`
impl Default for MatchTime {
    fn default() -> Self {
        Self::new()
    }
}
