use bevy::prelude::{Color, Vec2};


pub const BACKGROUND_COLOR: Color = Color::GRAY;  
pub const POINT_COLOR: Color = Color::BLACK;
pub const POINT_SIZE: f32 = 30.0;
pub const POINT_INTERNAL_SIZE: Vec2 = Vec2::new(20.0, 20.0);
pub const DOT_CENTER_DIFF: f32 = 1.5;
pub const POINT_SIZE_VEC: Vec2 = Vec2::new(POINT_SIZE, POINT_SIZE);