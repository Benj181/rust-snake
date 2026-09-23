use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, terminal,
};
use std::io::stdout;

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;

    let mut snake = Snake::new();
    loop {
        if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
            if let Ok(Event::Key(key)) = event::read() {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    execute!(stdout(), cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug)]
struct Snake {
    body: Vec<Coordinates>,
    direction: Direction,
}

impl Snake {
    fn new() -> Self {
        Self {
            body: vec![Coordinates::new()],
            direction: Direction::Up,
        }
    }

    fn get_head_position(&self) -> &Coordinates {
        &self.body[0]
    }
}
