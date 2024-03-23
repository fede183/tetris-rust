use bevy::prelude::*;
mod app;
mod classes;

use crate::classes::config::{DISPLAY_WIDTH, DISPLAY_HEIGTH};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (DISPLAY_WIDTH, DISPLAY_HEIGTH).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, app::setup)
        .run();
}
