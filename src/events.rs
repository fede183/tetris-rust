use bevy::prelude::*;
use crate::board::PieceComponent;


pub fn piece_input_system(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut PieceComponent>) {
    // let mut piece = query.single_mut();
    // if input.just_pressed(KeyCode::ArrowDown) {
    //     piece.descend();
    // }
}
