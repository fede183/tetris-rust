use bevy::prelude::*;
use crate::config::SQUARE_SIZE;
use crate::game::game_state::GameData;
use crate::board::BoardPieceComponent;


pub fn piece_input_system(
    input: Res<ButtonInput<KeyCode>>, 
    mut game_data: ResMut<GameData>,
    mut query: Query<(Entity, &mut Transform), With<BoardPieceComponent>>,
    ) {
    if input.just_pressed(KeyCode::ArrowDown) {
        game_data.descend();
        let (_, mut transform) = query.single_mut();
        transform.translation.y -= SQUARE_SIZE;
    }
}
