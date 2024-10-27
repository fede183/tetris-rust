use bevy::prelude::*;
use crate::event_bloker::*;
use crate::cycle_timer::*;
use crate::board::*;
use crate::common::*;
use crate::events::*;
use crate::score::*;
use crate::game::game_data::GameData;

pub struct InGamePlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for InGamePlugin<S> {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(GameData::new_game())
        .init_resource::<CycleTimer>()
        .init_resource::<EventBlocker>()
        .add_systems(Startup, (init_camera, init_board, init_board_pieces, init_score).run_if(in_state(self.state.clone())))
        .add_systems(Update, (update_score, update_lines, toggle_resolution, piece_input_system, cycle_system, toggle_game_over).run_if(in_state(self.state.clone())));
    }
}
