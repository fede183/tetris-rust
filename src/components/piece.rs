use bevy::prelude::{Component, Vec3, Color};

#[derive(Component)]
pub struct Piece;

pub static PIECE_COLOR: Color = Color::BLACK;
pub static PIECE_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);