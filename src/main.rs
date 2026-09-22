use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, terminal,
};
use std::io::stdout;

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;

    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
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
