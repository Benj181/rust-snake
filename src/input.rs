use crate::engine::Direction;
use crossterm::event;

impl Direction {
    pub fn from_key(&key: &event::KeyEvent) -> Option<Self> {
        match key.code {
            event::KeyCode::Char('w') => Some(Direction::Up),
            event::KeyCode::Char('s') => Some(Direction::Down),
            event::KeyCode::Char('d') => Some(Direction::Right),
            event::KeyCode::Char('a') => Some(Direction::Left),
            _ => None,
        }
    }
}
