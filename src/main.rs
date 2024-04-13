mod board;
mod config;
mod common;
mod score;
mod events;

use bevy::prelude::*;
use board::*;
use common::*;
use score::*;
use events::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (init_camera, init_score, init_board))
        .add_systems(Update, (setup_board, setup_next_piece_board, setup_score, toggle_resolution, piece_input_system))
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

