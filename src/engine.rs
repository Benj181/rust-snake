use crate::input::PlayerIntent;
use rand::seq::IndexedRandom;
use std::collections::VecDeque;
use std::fmt;
use std::num::NonZeroU16;

const STARTING_BODY_LENGTH: usize = 3;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    const fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Up => write!(f, "Up"),
            Self::Down => write!(f, "Down"),
            Self::Right => write!(f, "Right"),
            Self::Left => write!(f, "Left"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    x: u16,
    y: u16,
}

impl Coordinate {
    const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub const fn get_coordinate(self) -> (u16, u16) {
        (self.x, self.y)
    }

    const fn is_inside(self, grid: Grid) -> bool {
        self.x < grid.width.get() && self.y < grid.height.get()
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cords = self.get_coordinate();
        write!(f, "({}, {})", cords.0, cords.1)
    }
}

#[derive(Debug)]
pub struct Snake {
    head: Coordinate,
    body: VecDeque<Coordinate>,
    body_length: usize,
    /// The direction the snake last moved in.
    direction: Direction,
    /// The direction it will move in on the next tick.
    next_direction: Direction,
}

impl Snake {
    const fn new(start: Coordinate, length: usize) -> Self {
        Self {
            head: start,
            body: VecDeque::new(),
            body_length: length,
            direction: Direction::Down,
            next_direction: Direction::Down,
        }
    }

    fn next_head_position(&self) -> Option<Coordinate> {
        match self.next_direction {
            Direction::Up => Some(Coordinate {
                x: self.head.x,
                y: self.head.y.checked_sub(1)?,
            }),
            Direction::Down => Some(Coordinate {
                x: self.head.x,
                y: self.head.y.checked_add(1)?,
            }),
            Direction::Right => Some(Coordinate {
                x: self.head.x.checked_add(1)?,
                y: self.head.y,
            }),
            Direction::Left => Some(Coordinate {
                x: self.head.x.checked_sub(1)?,
                y: self.head.y,
            }),
        }
    }

    fn do_move(&mut self, next_head: Coordinate) {
        self.direction = self.next_direction;
        self.body.push_front(self.head);
        self.head = next_head;

        while self.body.len() > self.body_length {
            self.body.pop_back();
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        if self.direction.opposite() != new_direction {
            self.next_direction = new_direction;
        }
    }

    const fn grow(&mut self) {
        self.body_length = self.body_length.saturating_add(1);
    }

    fn occupies(&self, cell: Coordinate) -> bool {
        self.head == cell || self.body.contains(&cell)
    }

    fn has_collided_with_itself(&self) -> bool {
        self.body.contains(&self.head)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Grid {
    width: NonZeroU16,
    height: NonZeroU16,
}

impl Grid {
    pub const fn new((width, height): (u16, u16)) -> Option<Self> {
        match (NonZeroU16::new(width), NonZeroU16::new(height)) {
            (Some(width), Some(height)) => Some(Self { width, height }),
            _ => None,
        }
    }

    pub const fn width(self) -> u16 {
        self.width.get()
    }

    pub const fn height(self) -> u16 {
        self.height.get()
    }

    fn cells(self) -> impl Iterator<Item = Coordinate> {
        (0..self.height()).flat_map(move |y| (0..self.width()).map(move |x| Coordinate::new(x, y)))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Menu,
    Active,
    GameOver,
    Won,
    Pause,
}

#[derive(Debug)]
pub struct World {
    snake: Snake,
    grid: Grid,
    food: Option<Coordinate>,
    game_state: GameState,
}

impl World {
    pub fn new(grid: Grid) -> Self {
        let mut world = Self {
            snake: Snake::new(
                Coordinate::new(grid.width.get() / 2, grid.height.get() / 2),
                STARTING_BODY_LENGTH,
            ),
            grid,
            food: None,
            game_state: GameState::Active,
        };
        world.spawn_food();
        world
    }

    pub fn tick(&mut self) {
        if self.game_state != GameState::Active {
            return;
        }

        let Some(next_head) = self
            .snake
            .next_head_position()
            .filter(|cell| cell.is_inside(self.grid))
        else {
            self.game_state = GameState::GameOver;
            return;
        };

        let eats_food = self.food == Some(next_head);
        if eats_food {
            self.snake.grow();
        }

        // Move before checking for self-collision, so moving into the cell the tail
        // is leaving is allowed.
        self.snake.do_move(next_head);
        if self.snake.has_collided_with_itself() {
            self.game_state = GameState::GameOver;
        } else if eats_food {
            self.spawn_food();
        }
    }

    /// Places food on a random free cell, or wins the game if there is none left.
    fn spawn_food(&mut self) {
        let free_cells: Vec<Coordinate> = self
            .grid
            .cells()
            .filter(|&cell| !self.snake.occupies(cell))
            .collect();

        self.food = free_cells.choose(&mut rand::rng()).copied();
        if self.food.is_none() {
            self.game_state = GameState::Won;
        }
    }

    pub fn handle(&mut self, intent: PlayerIntent) {
        match intent {
            PlayerIntent::Move(dir) => self.snake.change_direction(dir),
            PlayerIntent::Pause => {
                self.game_state = match self.game_state {
                    GameState::Active => GameState::Pause,
                    GameState::Pause => GameState::Active,
                    _ => return,
                }
            }
            PlayerIntent::Quit => {}
        }
    }

    pub fn get_snake_cells(&self) -> impl Iterator<Item = Coordinate> + '_ {
        std::iter::once(self.snake.head).chain(self.snake.body.iter().copied())
    }

    /// The number of food eaten. Each one grows the body by one cell.
    pub const fn get_score(&self) -> usize {
        self.snake.body_length.saturating_sub(STARTING_BODY_LENGTH)
    }

    pub const fn get_food(&self) -> Option<Coordinate> {
        self.food
    }

    pub const fn get_grid(&self) -> Grid {
        self.grid
    }

    pub const fn get_game_state(&self) -> GameState {
        self.game_state
    }
}
