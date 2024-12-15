use rand_derive2::RandGen;
use rand;
use super::point::{Point, PointColor};
use std::f32::consts::PI;


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
            PieceType::Z => (1.5, 1.),
            PieceType::ReverseZ => (1.5, 1.),
            PieceType::L => (1.5, 1.),
            PieceType::ReverseL => (1.5, 1.),
            PieceType::Line => (2., 1.),
            PieceType::Cube => (1., 1.),
            PieceType::T => (1.5, 1.),
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
        self.center_point = (self.center_point.0 , self.center_point.1 + 1.);
    }
    
    pub fn move_right(&mut self) {
        for point in &mut self.points {
            point.move_right();
        }
        self.center_point = (self.center_point.0 + 1., self.center_point.1);
    }
    
    pub fn move_left(&mut self) {
        for point in &mut self.points {
            point.move_left();
        }
        self.center_point = (self.center_point.0 - 1., self.center_point.1);
    }
    
    pub fn rotate(&mut self) {

        let center_point = self.center_point;
        let center_point_x = center_point.0;
        let center_point_y = center_point.1;

        let beta: f32 = PI / 2.;

        for point in &mut self.points {
            let x_0 = point.x as f32 - center_point_x;
            let y_0 = -(point.y as f32 - center_point_y);

            let x_prima = x_0 * beta.cos() - y_0 * beta.sin();
            let y_prima = x_0 * beta.sin() + y_0 * beta.cos();

            point.x = (center_point_x + x_prima) as i32;
            point.y = (center_point_y - y_prima) as i32;
        }
    }
}
