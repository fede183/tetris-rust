use rand_derive2::RandGen;
use rand;
use rand::Rng;
use super::point::{Point, PointColor};


#[derive(Debug, RandGen)]
enum PieceType {
    Z,
    REVERSE_Z,
    L,
    REVERSE_L,
    LINE,
    CUBE,
    T,
}

impl PieceType {
    pub fn get_piece_coordinates(&self) -> [(i32, i32); 4] {
        match self {
			PieceType::Z => [(0, 0), (0, 1), (0, 2), (1, 2)],
			PieceType::REVERSE_Z => [(1, 0), (1, 1), (1, 2), (0, 2)],
			PieceType::L => [(0, 0), (1, 0), (1, 1), (2, 1)],
			PieceType::REVERSE_L => [(0, 1), (1, 0), (1, 1), (2, 0)],
			PieceType::LINE => [(0, 0), (1, 0), (2, 0), (3, 1)],
			PieceType::CUBE => [(0, 0), (0, 1), (1, 0), (1, 1)],
			PieceType::T => [(0, 0), (1, 0), (2, 0), (1, 1)],
        }
    } 
}

pub struct Piece {
    pub points: Vec<Point>
}

impl Piece {
    pub fn descend(&mut self) {
        for point in &mut self.points {
            point.descend();
        } 
    }
    pub fn ascend(&mut self) {
        for point in &mut self.points {
            point.ascend();
        } 
    }
    pub fn move_right(&mut self) {
        for point in &mut self.points {
            point.move_right();
        } 
    }
    pub fn move_left(&mut self) {
        for point in &mut self.points {
            point.move_left();
        } 
    }
    pub fn belongs(&self, other_point: &Point) -> bool {
        for point in &self.points {
            if point.equal(&other_point) {
                return true;
            }
        }
        false
    }
    pub fn generate_random_piece() -> Piece {
        let mut points = Vec::new();
        let piece_type: PieceType = rand::random();
        let coordinates_piece = piece_type.get_piece_coordinates();
        let color: PointColor = rand::random();
        for coordinate in coordinates_piece {
            let (x, y) = coordinate;
            points.push(Point { x, y, color: color.clone() });
        }

        Piece { points }
    }
    pub fn rorate(&mut self) {
        
    }
}

