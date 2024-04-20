use bevy::prelude::*;
use crate::game::piece::Piece;
use crate::BoardPieceComponent;
use crate::{board::PieceComponent, PointComponent};


pub fn piece_input_system(input: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut PieceComponent, &mut Transform)>) {
    // let mut piece = query.single_mut();
    // if input.just_pressed(KeyCode::ArrowDown) {
    //     piece.descend();
    // }
}
