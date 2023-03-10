use bevy::prelude::*;

use crate::{consts::{POINT_SIZE, POINT_COLOR, POINT_INTERNAL_SIZE, BACKGROUND_COLOR, DOT_CENTER_DIFF, POINT_SIZE_VEC}, entities::dot::Dot};

pub fn spawn_point(
    commands: &mut ChildBuilder,
    dot: &Dot
) { 

    let dot_first_item = dot.0 as f32 - DOT_CENTER_DIFF;
    let dot_second_item = dot.1 as f32 - DOT_CENTER_DIFF;

    let position_x = POINT_SIZE*dot_first_item;
    let position_y = POINT_SIZE*dot_second_item;

    let position = Vec3::new(position_x, position_y, 0.);

    let first_quad = SpriteBundle {
        sprite: Sprite {
            color: POINT_COLOR,
            custom_size: Some(POINT_SIZE_VEC),
            ..default()
        },
        transform: Transform::from_translation(position),
        ..default()
    };

    let second_quad = SpriteBundle {
        sprite: Sprite {
            color: BACKGROUND_COLOR,
            custom_size: Some(POINT_INTERNAL_SIZE),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        ..default()
    };

    commands.spawn(first_quad).with_children(|parent| {
        parent.spawn(second_quad);
    });
}