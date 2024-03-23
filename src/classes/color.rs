use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use bevy::prelude::Color;

pub enum PointColor {
    BLUE,
    VIOLET,
    RED,
    GREEN,
    YELLOW,
    CYAN,
    ORANGE,
    MAROON,
}

impl Distribution<PointColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PointColor {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=7) { // rand 0.8
            0 => PointColor::BLUE,
            1 => PointColor::VIOLET,
            2 => PointColor::RED,
            3 => PointColor::GREEN,
            4 => PointColor::YELLOW,
            5 => PointColor::CYAN,
            6 => PointColor::ORANGE,
            _ => PointColor::MAROON
        }
    }
}

impl PointColor {
    pub fn get_random_color() -> Color {
       let rand_color: PointColor = rand::random();

       match rand_color {
            PointColor::BLUE => Color::BLUE,
            PointColor::VIOLET => Color::VIOLET,
            PointColor::RED => Color::RED,
            PointColor::GREEN => Color::GREEN,
            PointColor::YELLOW => Color::YELLOW,
            PointColor::CYAN => Color::CYAN,
            PointColor::ORANGE => Color::ORANGE,
            PointColor::MAROON => Color::MAROON,
       }
    }
}
