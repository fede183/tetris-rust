mod board;
mod config;
mod common;
mod score;
mod events;
mod game;
mod sprites;
mod bevy_color_converter;

use bevy::prelude::*;
use board::*;
use common::*;
use game::game_state::{GameState, GameData};
use score::*;
use events::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_state(GameState::Playing)
        .insert_resource(GameData::new_game())
        .add_systems(Startup, (init_camera, init_score, init_board, init_board_pieces))
        .add_systems(Update, (setup_score, toggle_resolution, piece_input_system)
                     .run_if(in_state(GameState::Playing)))
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

