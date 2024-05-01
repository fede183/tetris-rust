use bevy::prelude::*;
use crate::game::piece::Piece;
use crate::{board::BoardPieceComponent, PointComponent};


pub fn piece_input_system(input: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut BoardPieceComponent, &mut Transform)>) {
    // let mut piece = query.single_mut();
    // if input.just_pressed(KeyCode::ArrowDown) {
    //     piece.descend();
    // }
}
