use bevy::prelude::*;
use crate::cycle_timer::*;
use crate::board::*;
use crate::event_bloker::EventBlocker;
use crate::events::*;
use crate::score::*;
use crate::config::*;
use crate::game::game_data::GameData;

pub struct InGamePlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for InGamePlugin<S> {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameData::new_game())
            .insert_resource(ClearColor(Color::BLACK))
            .init_resource::<CycleTimer>()
            .init_resource::<EventBlocker>()
            .add_plugins(DefaultPlugins)
            .add_systems(Startup, (init_camera, toggle_window, init_board, init_board_pieces, init_score))
            .add_systems(Update, (update_score, update_lines, piece_input_system, cycle_system, toggle_game_over)
                .run_if(in_state(self.state.clone())));
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn toggle_window(mut window: Query<&mut Window>) {
    let mut window_mut = window.single_mut();
    window_mut.title = "Tetris!".into();
    window_mut.name = Some("ingame.app".into());
    window_mut.resolution = (DISPLAY_WINDOW_WIGTH, DISPLAY_WINDOW_HEIGHT).into();
    window_mut.prevent_default_event_handling = false;
    window_mut.visible = true;
}

