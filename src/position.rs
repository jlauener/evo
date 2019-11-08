use std::fmt;
use std::ops;

pub const LEFT: Position = Position { x: -1, y: 0 };
pub const RIGHT: Position = Position { x: 1, y: 0 };
pub const UP: Position = Position { x: 0, y: -1 };
pub const DOWN: Position = Position { x: 0, y: 1 };
pub const LEFT_UP: Position = Position { x: -1, y: -1 };
pub const RIGHT_UP: Position = Position { x: 1, y: -1 };
pub const LEFT_DOWN: Position = Position { x: -1, y: 1 };
pub const RIGHT_DOWN: Position = Position { x: 1, y: 1 };

#[derive(Copy, Clone, Debug, Default)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Position {
    pub fn new(x: i16, y: i16) -> Position {
        Position { x, y }
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
