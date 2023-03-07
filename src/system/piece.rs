use bevy::prelude::*;

use crate::components::piece::{Piece, PIECE_SIZE, PIECE_COLOR};

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_piece);
    }
}

fn setup_piece(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 50.0, 0.0),
            scale: PIECE_SIZE,
            ..default()
        },
        sprite: Sprite {
            color: PIECE_COLOR,
            ..default()
        },
        ..default()
    }, 
    Piece
    ));
}

pub fn move_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Piece>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction_x = 0.0;
        let mut direction_y = 0.0;

        if keyboard_input.pressed(KeyCode::Left) {
            direction_x -= 1.0;
        }
    
        if keyboard_input.pressed(KeyCode::Right) {
            direction_x += 1.0;
        }
    
        if keyboard_input.pressed(KeyCode::Down) {
            direction_y -= 1.0;
        }
    
        if keyboard_input.pressed(KeyCode::Up) {
            direction_y += 1.0;
        }
    
        transform.translation.x += direction_x * 3.0;
        transform.translation.y += direction_y * 3.0;
    }

}
