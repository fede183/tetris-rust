use bevy::prelude::ResMut;
use bevy::time::Time;
use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct EventBlocker {
    time: f32,
    button_was_press: bool,
}


impl EventBlocker {
    pub fn new() -> Self {
        Self { time: 0.0, button_was_press: false }
    }

    pub fn is_block(&mut self, time: ResMut<Time>) -> bool {
        if !self.button_was_press {
            return false;
        }

        self.time += time.delta().as_secs_f32();
        if self.time >= 0.15 {
            self.button_was_press = false;
            self.time = 0.0;
            return false;
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
