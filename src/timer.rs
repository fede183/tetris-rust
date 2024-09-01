use bevy::prelude::ResMut;
use bevy::time::{Time, Timer, TimerMode};
use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct EventBlocker {
    timer: Option<Timer>,
    button_was_press: bool,
}


impl EventBlocker {
    pub fn new() -> Self {
        Self { timer: None , button_was_press: false }
    }

    pub fn is_block(&mut self, time: ResMut<Time>) -> bool {
        if !self.button_was_press {
            return false;
        }

        if self.timer == None {
            self.timer = Some(Timer::from_seconds(0.15, TimerMode::Once));
        }
        
        if let Some(timer_value) = &mut self.timer {

            timer_value.tick(time.delta());
            if timer_value.finished() {
                self.button_was_press = false;
                self.timer = None;
                return false;
            }
        }

        return true;
    }

    pub fn block(&mut self) {
        self.button_was_press = true;
    }
}

// We need to implement Default so that our timer can be initialized as
// a resource when we call `init_resource`
impl Default for EventBlocker {
    fn default() -> Self {
        Self::new()
    }
}
