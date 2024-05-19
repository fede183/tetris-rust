use bevy::prelude::*;
use crate::config::*;

pub fn toggle_resolution(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    window.resolution.set(DISPLAY_WINDOW_WIGTH, DISPLAY_WINDOW_HEIGHT);
}

pub fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
