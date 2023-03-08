use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::components::point::{POINT_COLOR, POINT_SIZE, Point};

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_point).add_system(move_point);
    }
}

fn setup_point(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let point = Mesh::from(shape::Quad::default());

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(point).into(),
        transform: Transform::default().with_scale(POINT_SIZE),
        material: materials.add(ColorMaterial::from(POINT_COLOR)),
        ..default()
    },
    Point));
}


pub fn move_point(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Point>>,
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