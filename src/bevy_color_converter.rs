use crate::game::point::PointColor;
use bevy::prelude::Color;

impl PointColor {
    pub fn get_color(&self) -> Color {
        match self {
            PointColor::RED => Color::RED,
            PointColor::BLUE => Color::BLUE,
            PointColor::YELLOW => Color::YELLOW,
            PointColor::GREEN => Color::GREEN,
        }
    }
}
