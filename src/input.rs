use crate::engine::Direction;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum PlayerIntent {
    Move(Direction),
    Pause,
    Quit,
}

impl PlayerIntent {
    const fn from_key(key: KeyEvent) -> Option<Self> {
        match key.code {
            KeyCode::Char('w') | KeyCode::Up => Some(Self::Move(Direction::Up)),
            KeyCode::Char('s') | KeyCode::Down => Some(Self::Move(Direction::Down)),
            KeyCode::Char('d') | KeyCode::Right => Some(Self::Move(Direction::Right)),
            KeyCode::Char('a') | KeyCode::Left => Some(Self::Move(Direction::Left)),
            KeyCode::Char('p') => Some(Self::Pause),
            KeyCode::Char('q') | KeyCode::Esc => Some(Self::Quit),
            _ => None,
        }
    }
}

pub fn poll(timeout: Duration) -> io::Result<Option<PlayerIntent>> {
    if !event::poll(timeout)? {
        return Ok(None);
    }
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => Ok(PlayerIntent::from_key(key)),
        _ => Ok(None),
    }
}
