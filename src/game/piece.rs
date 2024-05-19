use rand_derive2::RandGen;
use rand;
use super::point::{Point, PointColor};


#[derive(Debug, RandGen)]
enum PieceType {
    Z,
    ReverseZ,
    L,
    ReverseL,
    Line,
    Cube,
    T,
}

impl PieceType {
    pub fn get_piece_coordinates(&self) -> [(i32, i32); 4] {
        match self {
			PieceType::Z => [(0, 0), (0, 1), (0, 2), (1, 2)],
			PieceType::ReverseZ => [(1, 0), (1, 1), (1, 2), (0, 2)],
			PieceType::L => [(0, 0), (1, 0), (1, 1), (2, 1)],
			PieceType::ReverseL => [(0, 1), (1, 0), (1, 1), (2, 0)],
			PieceType::Line => [(0, 0), (1, 0), (2, 0), (3, 0)],
			PieceType::Cube => [(0, 0), (0, 1), (1, 0), (1, 1)],
			PieceType::T => [(0, 0), (1, 0), (2, 0), (1, 1)],
        }
    }
}

#[derive(Clone)]
struct PieceRotation(u16);

impl PieceRotation {
    fn new() -> PieceRotation {
        PieceRotation { 0: 0 }
    }

    fn rotate(&mut self) {
        if (self.0 == 360) {
            self.0 = 0;
        } else {
            self.0 += 90;
        }
    }
}

#[derive(Clone)]
pub struct Piece {
    pub points: Vec<Point>,
    pub rotation: PieceRotation,
}

impl Piece {
    pub fn generate_random_piece() -> Piece {
        let mut points = Vec::new();
        let piece_type: PieceType = rand::random();
        let coordinates_piece = piece_type.get_piece_coordinates();
        let color: PointColor = rand::random();
        for coordinate in coordinates_piece {
            let (x, y) = coordinate;
            points.push(Point { x, y, color: color.clone() });
        }

        Piece { points, rotation: PieceRotation::new() }
    }

    pub fn descend(&mut self) {
        for point in &mut self.points {
            point.descend();
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
    
    pub fn rotate(&mut self) {
        self.rotation = ;

        let points = self.points.clone();
        let center_point = points.get(0).expect("Invalid piece");

        for point in &mut self.points {
            let rotate_x = point.y - center_point.y;
            let rotate_y = point.x - center_point.x;

            point.x = center_point.x - rotate_x;
            point.y = center_point.y - rotate_y;
        }
    }
    
}
