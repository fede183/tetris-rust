use super::brick::{Brick, Dot};

use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use rand::{prelude::*, distributions::Uniform};

#[derive(EnumIter)]
pub enum BrickTypes {
    Line,
    Square,
    T,
    Z,
    ZReverse,
    L,
    LReverse,
}

impl BrickTypes {

    // x 0 1 2 3
    // 0 x x x x
    // 1 x x x x
    // 2 x x x x
    // 3 x x x x

    pub fn get_brick(&self) -> Brick {
        match self {
            BrickTypes::Line => Brick {dots: [Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(3, 1)], rotation_center: 1.5 },
            BrickTypes::Square => Brick {dots: [Dot(1, 1), Dot(2, 1), Dot(1, 2), Dot(2, 2)], rotation_center: 1.5 },
            BrickTypes::T => Brick {dots: [Dot(0, 1), Dot(1, 0), Dot(1, 1), Dot(1, 2)], rotation_center: 1.0 },
            BrickTypes::Z => Brick {dots: [Dot(0, 0), Dot(0, 1), Dot(1, 1), Dot(1, 2)], rotation_center: 0.5 },
            BrickTypes::ZReverse => Brick {dots: [Dot(1, 0), Dot(1, 1), Dot(0, 1), Dot(0, 2)], rotation_center: 0.5 },
            BrickTypes::L => Brick {dots: [Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(2, 2)], rotation_center: 1.0 },
            BrickTypes::LReverse => Brick {dots: [Dot(2, 0), Dot(0, 1), Dot(1, 1), Dot(2, 1)], rotation_center: 1.0 },
        }
    }

    pub fn get_random_brick() -> Brick {
        let array_brick_types = BrickTypes::iter().collect::<Vec<BrickTypes>>();
        let mut rng = rand::thread_rng();

        let random_index = rng.sample::<usize, _>(Uniform::new(0, array_brick_types.len()));

        let random_brick_type = &array_brick_types[random_index];

        random_brick_type.get_brick()
    }
}