mod board;
mod config;
mod common;
mod score;

use bevy::prelude::*;
use board::*;
use common::*;
use score::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (init_camera, init_score))
        .add_systems(Update, (setup_board, setup_next_piece_board, setup_score, toggle_resolution))
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

