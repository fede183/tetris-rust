mod board;
mod config;
mod common;
mod score;
mod events;
mod game;
mod sprites;
mod bevy_color_converter;
mod event_bloker;
mod cycle_timer;

use bevy::prelude::*;
use board::*;
use common::*;
use game::game_state::GameState;
use game::game_data::GameData;
use score::*;
use events::*;
use event_bloker::*;
use cycle_timer::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_state(GameState::Playing)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(GameData::new_game())
        .init_resource::<CycleTimer>()
        .init_resource::<EventBlocker>()
        .add_systems(Startup, (init_camera, init_board, init_board_pieces, init_score))
        .add_systems(Update, (update_score, update_lines, toggle_resolution, piece_input_system, cycle_system)
        .run_if(in_state(GameState::Playing)))
        .run();
}

