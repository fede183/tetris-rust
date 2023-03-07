mod components;
mod entities;
mod system;

use bevy::{prelude::*, window::PresentMode};

use system::{piece::{move_piece, PiecePlugin}, setup::setup};

static BACKGROUND_COLOR: Color = Color::WHITE;

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                resolution: (800., 1000.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(PiecePlugin)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(move_piece)
        .run();
}