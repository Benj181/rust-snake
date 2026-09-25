use crate::engine::{Coordinate, GameState, Grid, World};
use crossterm::cursor::{self, MoveTo};
use crossterm::style::{Print, PrintStyledContent, StyledContent, Stylize};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use std::io::{self, Write};

const CELL_WIDTH: u16 = 2;
const BORDER: u16 = 1;

const CELL: &str = "██";

pub const fn grid_size((columns, rows): (u16, u16)) -> (u16, u16) {
    let inner_columns = columns.saturating_sub(BORDER.saturating_mul(2));
    let inner_rows = rows.saturating_sub(BORDER.saturating_mul(2));
    (inner_columns / CELL_WIDTH, inner_rows)
}

pub struct Renderer<W: Write> {
    out: W,
}

impl<W: Write> Renderer<W> {
    pub fn new(out: W) -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let mut renderer = Self { out };
        execute!(renderer.out, EnterAlternateScreen, cursor::Hide)?;
        Ok(renderer)
    }

    pub fn draw(&mut self, world: &World) -> io::Result<()> {
        queue!(self.out, Clear(ClearType::All))?;
        self.draw_border(world.get_grid())?;
        self.draw_score(world)?;
        for &food in world.get_food() {
            self.draw_cell(food, CELL.red())?;
        }
        self.draw_snake(world)?;
        self.draw_overlay(world)?;
        self.out.flush()
    }

    fn draw_border(&mut self, grid: Grid) -> io::Result<()> {
        let inner_width = grid.width().saturating_mul(CELL_WIDTH);
        let horizontal = "─".repeat(usize::from(inner_width));
        let right = inner_width.saturating_add(BORDER);
        let bottom = grid.height().saturating_add(BORDER);

        queue!(
            self.out,
            MoveTo(0, 0),
            Print(format!("┌{horizontal}┐")),
            MoveTo(0, bottom),
            Print(format!("└{horizontal}┘")),
        )?;
        for row in BORDER..bottom {
            queue!(
                self.out,
                MoveTo(0, row),
                Print('│'),
                MoveTo(right, row),
                Print('│')
            )?;
        }
        Ok(())
    }

    /// Writes the score into the top border, leaving the corners intact.
    fn draw_score(&mut self, world: &World) -> io::Result<()> {
        let text = format!(" Score: {} ", world.get_score());
        let inner_width = world.get_grid().width().saturating_mul(CELL_WIDTH);
        let fits = u16::try_from(text.chars().count()).is_ok_and(|width| width <= inner_width);
        if !fits {
            return Ok(());
        }
        queue!(self.out, MoveTo(BORDER, 0), PrintStyledContent(text.bold()))
    }

    fn draw_snake(&mut self, world: &World) -> io::Result<()> {
        let mut cells = world.get_snake_cells();
        if let Some(head) = cells.next() {
            self.draw_cell(head, CELL.yellow())?;
        }
        for cell in cells {
            self.draw_cell(cell, CELL.green())?;
        }
        Ok(())
    }

    fn draw_cell(&mut self, cell: Coordinate, content: StyledContent<&str>) -> io::Result<()> {
        let (x, y) = cell.get_coordinate();
        let column = x.saturating_mul(CELL_WIDTH).saturating_add(BORDER);
        let row = y.saturating_add(BORDER);
        queue!(self.out, MoveTo(column, row), PrintStyledContent(content))
    }

    fn draw_overlay(&mut self, world: &World) -> io::Result<()> {
        let (title, hint) = match world.get_game_state() {
            GameState::Pause => ("PAUSED", "p to resume, q to quit"),
            GameState::GameOver => ("GAME OVER", "q to quit"),
            GameState::Won => ("YOU WON", "q to quit"),
            GameState::Menu | GameState::Active => return Ok(()),
        };

        let grid = world.get_grid();
        let row = (grid.height() / 2).saturating_add(BORDER);
        self.print_centered(grid, row, title.bold())?;
        self.print_centered(grid, row.saturating_add(1), hint.dim())
    }

    fn print_centered(
        &mut self,
        grid: Grid,
        row: u16,
        text: StyledContent<&str>,
    ) -> io::Result<()> {
        let board_width = grid
            .width()
            .saturating_mul(CELL_WIDTH)
            .saturating_add(BORDER.saturating_mul(2));
        let text_width = u16::try_from(text.content().chars().count()).unwrap_or(board_width);
        let column = board_width.saturating_sub(text_width) / 2;
        queue!(self.out, MoveTo(column, row), PrintStyledContent(text))
    }
}

impl<W: Write> Drop for Renderer<W> {
    fn drop(&mut self) {
        let _ = execute!(self.out, cursor::Show, LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}
