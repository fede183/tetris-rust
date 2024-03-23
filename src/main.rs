mod board;
mod config;

use bevy::prelude::*;
use board::*;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_board)
        .run();
}

