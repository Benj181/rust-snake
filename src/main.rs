mod engine;
mod input;
mod render;

use crate::engine::{Direction, World};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue,
    style::Print,
    terminal,
};
use std::io::{Write, stdout};
use std::time::{Duration, Instant};

struct TerminalGuard;

impl TerminalGuard {
    fn new() -> std::io::Result<Self> {
        terminal::enable_raw_mode()?;
        let guard = Self;
        execute!(stdout(), cursor::Hide)?;
        Ok(guard)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(stdout(), cursor::Show);
        let _ = terminal::disable_raw_mode();
    }
}

fn main() -> std::io::Result<()> {
    let _guard = TerminalGuard::new()?;

    let update_interval = Duration::from_millis(150);
    let last_move = Instant::now();

    loop {
        let poll_timeout = update_interval.saturating_sub(last_move.elapsed());
        if matches!(event::poll(poll_timeout), Ok(true))
            && let Ok(Event::Key(key)) = event::read()
        {
            break;
        }

        // if Instant::now() - last_move >= update_interval {
        //     snake.do_move();
        //     last_move = Instant::now();
        // }
    }

    Ok(())
}
