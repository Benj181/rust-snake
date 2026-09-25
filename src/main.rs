mod engine;
mod input;
mod render;

use crate::engine::{Grid, World};
use crate::input::PlayerIntent;
use crate::render::Renderer;
use crossterm::terminal;
use std::io::{self, BufWriter};
use std::time::{Duration, Instant};

fn main() -> io::Result<()> {
    let mut renderer = Renderer::new(BufWriter::new(io::stdout()))?;

    let update_interval = Duration::from_millis(150);
    let mut last_move = Instant::now();

    let grid = Grid::new(render::grid_size(terminal::size()?))
        .ok_or_else(|| io::Error::other("terminal is too small"))?;
    let mut world = World::new(grid);
    renderer.draw(&world)?;

    loop {
        let poll_timeout = update_interval.saturating_sub(last_move.elapsed());
        if let Some(intent) = input::poll(poll_timeout)? {
            match intent {
                PlayerIntent::Quit => break,
                other => {
                    world.handle(other);
                    renderer.draw(&world)?;
                }
            }
        }

        if last_move.elapsed() >= update_interval {
            world.tick();
            renderer.draw(&world)?;
            last_move = Instant::now();
        }
    }

    Ok(())
}
