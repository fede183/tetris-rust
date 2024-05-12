use std::borrow::{Borrow, BorrowMut};

use bevy::prelude::*;
use crate::game::game_state::GameData;
use crate::board::BoardPieceComponent;
use crate::config::SQUARE_SIZE;

pub fn piece_input_system(
    input: Res<ButtonInput<KeyCode>>, 
    mut game_data: ResMut<GameData>,
    mut q_parent: Query<(&BoardPieceComponent, &Children)>,
    mut q_child: Query<&mut Transform>,
    ) {
    // let (_, mut transform) = query.single_mut();

    // if input.just_pressed(KeyCode::ArrowDown) {
    //     if game_data.descend() {
    //         transform.translation.y -= SQUARE_SIZE;
    //     }
    // }
    // if input.just_pressed(KeyCode::ArrowLeft) {
    //     if game_data.move_left() {
    //         transform.translation.x -= SQUARE_SIZE;
    //     }
    // }
    // if input.just_pressed(KeyCode::ArrowRight) {
    //     if game_data.move_right() {
    //         transform.translation.x += SQUARE_SIZE;
    //     }
    // }
    // if input.just_pressed(KeyCode::Space) {
    //     if game_data.rotate() {
    //         transform.rotate_z(0.9);
    //     }
    // }
    //
    if input.just_pressed(KeyCode::ArrowDown) {
        desend(game_data, q_parent, q_child);
    }
}

fn desend(
    mut game_data: ResMut<GameData>,
    mut q_parent: Query<(&BoardPieceComponent, &Children)>,
    mut q_child: Query<&mut Transform>,
) {
    if game_data.descend() {
        let descend_fn = |transform: &mut Transform| { transform.translation.y -= SQUARE_SIZE };

        apply_piece_sprites_from_child(q_parent, q_child, descend_fn);
    }
}


fn apply_piece_sprites_from_child(
    mut q_parent: Query<(&BoardPieceComponent, &Children)>,
    mut q_child: Query<&mut Transform>,
    function_to_apply: fn(&mut Transform),
) {

    let (_, children) = q_parent.single_mut();
    for &child in children.iter() {
        let sprite_transform = q_child.get_mut(child);
        match sprite_transform {
            Ok(mut transform) => {
                function_to_apply(&mut transform);
            },
            Err(_) => panic!("Not fond tranform"),
        };
    }
}
