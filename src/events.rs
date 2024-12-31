use bevy::prelude::*;
use crate::game::game_data::GameData;
use crate::config::SQUARE_SIZE;
use crate::board::spawn_piece;
use crate::board::spawn_next_piece;
use crate::board::spawn_remaining_points;
use crate::game::game_state::GameState;
use crate::BoardPieceComponent;
use crate::utils::cycle_timer::CycleTimer;
use crate::utils::event_bloker::EventBlocker;
use crate::NextPieceComponent;
use crate::RemainingPointsComponent;

pub fn piece_input_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
    mut event_blocker: ResMut<EventBlocker>,
    time: ResMut<Time>,
    mut query_piece_transformation: Query<(Entity, &mut Transform), With<BoardPieceComponent>>,
    mut query_next_piece_transformation: Query<Entity, With<NextPieceComponent>>,
    mut query_remainings_transformation: Query<Entity, With<RemainingPointsComponent>>,
    ) {
    
    event_blocker.timer.tick(time.delta());

    if !(key_pressed(&input, KeyCode::ArrowDown) || 
        key_pressed(&input, KeyCode::ArrowLeft) || 
        key_pressed(&input, KeyCode::ArrowRight) ||
        key_pressed(&input, KeyCode::Space)) {
        return;
    }

    if query_piece_transformation.is_empty() {
        return;
    } 

    if !event_blocker.timer.finished() {
        return;
    }
    event_blocker.timer.reset();

    let (entity, mut transform) = query_piece_transformation.single_mut();

    if key_pressed(&input, KeyCode::ArrowDown) {
        if game_data.descend() {
            transform.translation.y -= SQUARE_SIZE;
        } else {
            let entity_next = query_next_piece_transformation.single_mut();
            let entity_remainings = query_remainings_transformation.single_mut();
            respawn_components(&mut commands, &game_data, entity, entity_next, entity_remainings);
        }
    }
    
    if key_pressed(&input, KeyCode::ArrowLeft) {
        if game_data.move_left() {
            transform.translation.x -= SQUARE_SIZE;
        }
    }
    if key_pressed(&input, KeyCode::ArrowRight) {
        if game_data.move_right() {
            transform.translation.x += SQUARE_SIZE;
        }
    }
    if input.just_pressed(KeyCode::Space) {
        if game_data.rotate() {
            let entity_next = query_next_piece_transformation.single_mut();
            let entity_remainings = query_remainings_transformation.single_mut();
            respawn_components(&mut commands, &game_data, entity, entity_next, entity_remainings);
        }
    }
}

pub fn cycle_system(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut cycle_system: ResMut<CycleTimer>,
    time: ResMut<Time>,
    mut query_piece_transformation: Query<(Entity, &mut Transform), With<BoardPieceComponent>>,
    mut query_next_piece_transformation: Query<Entity, With<NextPieceComponent>>,
    mut query_remainings_transformation: Query<Entity, With<RemainingPointsComponent>>,
    ) {
    cycle_system.timer.tick(time.delta());

    // if it finished, move down
    if cycle_system.timer.finished() && cycle_system.timer.just_finished() {
        let (entity, mut _transform) = query_piece_transformation.single_mut();
        game_data.cycle();
        let entity_next = query_next_piece_transformation.single_mut();
        let entity_remainings = query_remainings_transformation.single_mut();
        respawn_components(&mut commands, &game_data, entity, entity_next, entity_remainings);
    }
}

pub fn toggle_game_over(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    game_data: ResMut<GameData>,
) {
    if *state.get() == GameState::Playing && game_data.is_game_over() {
        next_state.set(GameState::GameOver);
    }
}

fn respawn_components(commands: &mut Commands, game_data: &ResMut<GameData>, entity: Entity, entity_next: Entity, entity_remainings: Entity) {
    commands.entity(entity).despawn_recursive();
    commands.entity(entity_next).despawn_recursive();
    commands.entity(entity_remainings).despawn_recursive();
    spawn_piece(commands, &game_data);
    spawn_next_piece(commands, &game_data);
    spawn_remaining_points(commands, &game_data);
}

fn key_pressed(
    input: &Res<ButtonInput<KeyCode>>,
    key_code: KeyCode
    ) -> bool {
    input.just_pressed(key_code) || input.pressed(key_code)
}

