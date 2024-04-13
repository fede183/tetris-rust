use bevy::prelude::*;
use crate::board::Piece;


pub fn piece_input_system(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Piece>) {
    let mut piece = query.single_mut();
    if input.just_pressed(KeyCode::ArrowDown) {
        piece.descend();
    }
}
