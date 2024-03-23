use crate::classes::color::PointColor;
use crate::classes::point_on_board::PointOnBoard;
use rand::{thread_rng, Rng};
use bevy::prelude::Color;


pub struct Piece
{
    positions: [PointOnBoard; 4]
}

impl Piece {
    pub fn new() -> Self {
        let mut positions: [PointOnBoard; 4] = [PointOnBoard {x: 0, y: 0, color: Color::BLUE}; 4];
        let color: Color = PointColor::get_random_color();

        let figures: [[i32; 4]; 7] = [
            [0, 2, 4, 6], //I
            [1, 2, 3, 4], //Z
            [0, 2, 3, 5], //S
            [0, 2, 4, 5], //L
            [1, 2, 3, 5], //T
            [0, 1, 2, 3], //O
            [1, 3, 4, 5] //J
        ];
        
        let mut rng = thread_rng();

        let figure_chosed = figures[rng.gen_range(0..=7)];

        for index in 0..3 {
            positions[index].x = figure_chosed[index] % 2;
            positions[index].y = figure_chosed[index] / 2;
            positions[index].color = color;
        }

        Self { positions }
    }
}
