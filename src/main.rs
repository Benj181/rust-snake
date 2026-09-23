mod engine;
mod input;

use crate::engine::Direction;
use crate::engine::Snake;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue,
    style::Print,
    terminal,
};
use std::io::{Write, stdout};
use std::time::{Duration, Instant};

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;

    let update_interval = Duration::from_millis(150);
    let mut last_move = Instant::now();

    let mut snake = Snake::new(5);

    loop {
        let poll_timeout = update_interval.saturating_sub(Instant::now() - last_move);
        if let Ok(true) = event::poll(std::time::Duration::from_secs_f64(
            poll_timeout.as_secs_f64(),
        )) {
            if let Ok(Event::Key(key)) = event::read() {
                if let Some(direction) = Direction::from_key(&key) {
                    snake.change_direction(direction);
                }
                queue!(stdout(), cursor::MoveTo(0, 0), Print(snake.direction))?;
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if Instant::now() - last_move >= update_interval {
            snake.do_move();
            last_move = Instant::now();
            queue!(stdout(), cursor::MoveTo(0, 0), Print(snake.direction))?;
        }
        que
        stdout().flush()?;
    }

    execute!(stdout(), cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
