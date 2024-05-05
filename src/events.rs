use bevy::prelude::*;
use crate::game::game_state::GameData;
use crate::board::BoardPieceComponent;
use crate::config::SQUARE_SIZE;

pub fn piece_input_system(
    input: Res<ButtonInput<KeyCode>>, 
    mut game_data: ResMut<GameData>,
    mut query: Query<(Entity, &mut Transform), With<BoardPieceComponent>>,
    ) {
    let (_, mut transform) = query.single_mut();

    if input.just_pressed(KeyCode::ArrowDown) {
        if game_data.descend() {
            transform.translation.y -= SQUARE_SIZE;
        }
    }
    if input.just_pressed(KeyCode::ArrowLeft) {
        if game_data.move_left() {
            transform.translation.x -= SQUARE_SIZE;
        }
    }
    if input.just_pressed(KeyCode::ArrowRight) {
        if game_data.move_right() {
            transform.translation.x += SQUARE_SIZE;
        }
    }
}
