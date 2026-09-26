mod engine;
mod input;
mod render;

use crate::engine::{Grid, World};
use macroquad::prelude::*;

#[macroquad::main("Snake")]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await;
    }
}

// fn main() -> io::Result<()> {
//     let mut renderer = Renderer::new(BufWriter::new(io::stdout()))?;
//
//     let update_interval = Duration::from_millis(100);
//     let mut last_move = Instant::now();
//
//     let grid = Grid::new(render::grid_size(terminal::size()?))
//         .ok_or_else(|| io::Error::other("terminal is too small"))?;
//     let mut world = World::new(grid);
//     renderer.draw(&world)?;
//
//     loop {
//         let poll_timeout = update_interval.saturating_sub(last_move.elapsed());
//         if let Some(intent) = input::poll(poll_timeout)? {
//             match intent {
//                 PlayerIntent::Quit => break,
//                 other => {
//                     world.handle(other);
//                     renderer.draw(&world)?;
//                 }
//             }
//         }
//
//         if last_move.elapsed() >= update_interval {
//             world.tick();
//             renderer.draw(&world)?;
//             let next = last_move
//                 .checked_add(update_interval)
//                 .unwrap_or_else(Instant::now);
//             last_move = if next.elapsed() >= update_interval {
//                 Instant::now()
//             } else {
//                 next
//             };
//         }
//     }
//
//     Ok(())
// }
