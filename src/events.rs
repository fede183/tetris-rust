use bevy::prelude::*;
use crate::game::game_data::GameData;
use crate::config::SQUARE_SIZE;
use crate::board::spawn_piece;
use crate::board::spawn_next_piece;
use crate::board::spawn_remaining_points;
use crate::BoardPieceComponent;
use crate::NextPieceComponent;
use crate::RemainingPointsComponent;

pub fn piece_input_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
    mut query_piece_transformation: Query<(Entity, &mut Transform), With<BoardPieceComponent>>,
    mut query_next_piece_transformation: Query<Entity, With<NextPieceComponent>>,
    mut query_remainings_transformation: Query<Entity, With<RemainingPointsComponent>>,
    ) {
    let (entity, mut transform) = query_piece_transformation.single_mut();

    if input.just_pressed(KeyCode::ArrowDown) {
        if game_data.descend() {
            transform.translation.y -= SQUARE_SIZE;
        } else {
            let entity_next = query_next_piece_transformation.single_mut();
            let entity_remainings = query_remainings_transformation.single_mut();
            commands.entity(entity).despawn_recursive();
            commands.entity(entity_next).despawn_recursive();
            commands.entity(entity_remainings).despawn_recursive();
            spawn_piece(&mut commands, &game_data);
            spawn_next_piece(&mut commands, &game_data);
            spawn_remaining_points(&mut commands, &game_data);
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
    if input.just_pressed(KeyCode::Space) {
        if game_data.rotate() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
