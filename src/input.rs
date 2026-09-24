use crate::engine::Direction;
use crossterm::event;

impl Direction {
    pub const fn from_key(&key: &event::KeyEvent) -> Option<Self> {
        match key.code {
            event::KeyCode::Char('w') => Some(Self::Up),
            event::KeyCode::Char('s') => Some(Self::Down),
            event::KeyCode::Char('d') => Some(Self::Right),
            event::KeyCode::Char('a') => Some(Self::Left),
            _ => None,
        }
    }
}

enum PlayerIntent {
    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,
    Quit,
}
