use rand_derive2::RandGen;
use rand;

#[derive(Debug, RandGen, Clone, Copy)]
pub enum PointColor {
    RED,
    BLUE,
    YELLOW,
    GREEN,
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub color: PointColor,
}

impl Point {
    pub fn descend(&mut self) {
        self.y += 1; 
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
