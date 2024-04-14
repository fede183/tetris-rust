use rand::Rng;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn descend(&mut self) {
        self.y += 1; 
    }
    pub fn ascend(&mut self) {
        self.y -= 1; 
    }
    pub fn move_right(&mut self) {
        self.x += 1;
    }
    pub fn move_left(&mut self) {
        self.x -= 1;
    }
    pub fn equal(&self, point: &Point) -> bool {
        self.x == point.x && self.x == point.y
    }
}

pub struct Piece {
    points: Vec<Point>
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
        let mut rng = rand::thread_rng();
        let index: usize = rng.gen_range(0..=4);
        let figures = [
            [(0, 0), (0, 1), (0, 2), (1, 2)],
            [(1, 0), (1, 1), (1, 2), (0, 2)],
            [(0, 0), (1, 0), (1, 1), (2, 1)],
            [(0, 1), (1, 0), (1, 1), (2, 0)],
            [(0, 0), (0, 1), (1, 0), (1, 1)],
            [(0, 0), (1, 0), (2, 0), (3, 1)],
        ];

        let mut points = Vec::new();
        for index_in_figure in 0..4 {
            let (x, y) = figures[index][index_in_figure];
            points.push(Point { x, y });
        }

        Piece { points }
    }
    pub fn rorate(&mut self) {
        
    }
}

pub struct RemainingPoints(Vec<Point>);
