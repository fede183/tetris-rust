use bevy::prelude::*;
mod app;
mod classes;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, app::setup)
        .run();
}
