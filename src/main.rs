mod board;
mod config;
mod score;
mod events;
mod game;
mod sprites;
mod bevy_color_converter;
mod event_bloker;
mod cycle_timer;
mod plugins;

use bevy::prelude::*;
use board::*;
use game::game_state::GameState;
use plugins::{game_over_plugin::GameOverPlugin, in_game_plugin::InGamePlugin};


fn main() {
    App::new()
        .insert_state(GameState::Playing)
        .add_plugins(InGamePlugin {
            state: GameState::Playing,
        })
        .add_plugins(GameOverPlugin {
            state: GameState::GameOver,
        })
        .run();
}

