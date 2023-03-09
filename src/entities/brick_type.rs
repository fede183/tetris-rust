use super::brick::{Brick, Dot};

use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use rand::{prelude::*, distributions::Uniform};

#[derive(EnumIter)]
pub enum BrickTypes {
    Line,
    Square,
}

impl BrickTypes {
    pub fn get_brick(&self) -> Brick {
        match self {
            BrickTypes::Line => Brick {dots: [Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(3, 1)] },
            BrickTypes::Square => Brick {dots: [Dot(1, 1), Dot(2, 1), Dot(1, 2), Dot(2, 2)] },
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