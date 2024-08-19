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
    pub fn get_initial_piece_coordinates(&self) -> [(i32, i32); 4] {
        match self {
            PieceType::Z => [(0, 0), (1, 0), (1, 1), (2, 1)],
            PieceType::ReverseZ => [(0, 1), (1, 1), (1, 0), (2, 0)],
            PieceType::L => [(0, 0), (1, 0), (2, 0), (0, 1)],
            PieceType::ReverseL => [(0, 0), (1, 0), (2, 0), (2, 1)],
            PieceType::Line => [(0, 0), (1, 0), (2, 0), (3, 0)],
            PieceType::Cube => [(0, 0), (0, 1), (1, 0), (1, 1)],
            PieceType::T => [(0, 0), (1, 0), (2, 0), (1, 1)],
        }
    }

    pub fn get_center_point_coordinates(&self) -> (f32, f32) {
        match self {
            PieceType::Z => (1.0, 0.5),
            PieceType::ReverseZ => (1.0, 0.5),
            PieceType::L => (1.0, 0.0),
            PieceType::ReverseL => (1.0, 0.0),
            PieceType::Line => (1.5, 0.0),
            PieceType::Cube => (0.5, 0.5),
            PieceType::T => (1.0, 0.0),
        }
    }
}

#[derive(Clone)]
pub struct Piece {
    pub points: Vec<Point>,
    pub center_point: (f32, f32),
}

impl Piece {
    pub fn generate_random_piece() -> Piece {
        let piece_type: PieceType = rand::random();
        let color: PointColor = rand::random();
        let mut points = Vec::new();

        let coordinates_piece = piece_type.get_initial_piece_coordinates();
        for coordinate in coordinates_piece {
            let (x, y) = coordinate;
            points.push(Point { x, y, color: color.clone() });
        }

        Piece { points, center_point: piece_type.get_center_point_coordinates() }
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

        let (x, y) = self.center_point;

        for point in &mut self.points {
            let rotate_x = (point.y as f32 - x).abs();
            let rotate_y = (point.x as f32 - y).abs();

            point.x = (x - rotate_x).abs() as i32;
            point.y = (y - rotate_y).abs() as i32;
        }
    }
}
