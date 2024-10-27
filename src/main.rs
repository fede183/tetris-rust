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
mod plugins;

use bevy::prelude::*;
use board::*;
use game::game_state::GameState;
use plugins::in_game_plugin::InGamePlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_state(GameState::Playing)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(InGamePlugin {
            state: GameState::Playing,
        })
        .run();
}

