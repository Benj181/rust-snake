use crate::engine::Coordinate;
use crate::engine::Direction;
use std::fmt;

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Right => write!(f, "Right"),
            Direction::Left => write!(f, "Left"),
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cords = self.get_coordinate();
        write!(f, "({}, {})", cords.0, cords.1)
    }
}
