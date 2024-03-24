mod board;
mod config;
mod common;

use bevy::prelude::*;
use board::*;
use common::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_board)
        .add_systems(Update, toggle_resolution)
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .run();
}

