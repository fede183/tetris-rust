use bevy::prelude::*;

use crate::{components::piece::Piece, entities::{brick::Brick, dot::Dot}};

use super::point::spawn_point;

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spaw_piece).add_system(move_piece);
    }
}

fn spaw_piece(
    mut commands: Commands,
) { 

    let a_piece = SpriteBundle {
        transform: Transform::from_translation(Vec3::new(-30., 0., 0.)),
        ..default()
    };

    let brick = Brick {dots: [Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(3, 1)] };

    commands.spawn((a_piece, Piece))
    .with_children(|child| {
        (0..4).for_each(|i| {
            spawn_point(child, &brick.dots[i])
        });
    });
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