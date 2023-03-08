use bevy::prelude::{Component, Vec3, Color};

#[derive(Component)]
pub struct Point;

pub static POINT_COLOR: Color = Color::BLACK;
pub static POINT_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);