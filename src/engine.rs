use std::collections::VecDeque;
use std::fmt;

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

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    x: u16,
    y: u16,
}

impl Coordinate {
    const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    const fn get_coordinate(self) -> (u16, u16) {
        (self.x, self.y)
    }

    fn is_inside(self, grid: Grid) -> bool {
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
struct Snake {
    head: Coordinate,
    body: VecDeque<Coordinate>,
    body_length: usize,
    direction: Direction,
}

impl Snake {
    pub const fn new(start: Coordinate, length: usize) -> Self {
        Self {
            head: start,
            body: VecDeque::new(),
            body_length: length,
            direction: Direction::Down,
        }
    }

    // pub fn get_head_position(&self) -> &Coordinate {
    //     &self.head
    // }
    //
    // pub const fn get_direction(&self) -> Direction {
    //     self.direction
    // }
    pub fn next_head_position(&self) -> Option<Coordinate> {
        match self.direction {
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

    pub fn do_move(&mut self, next_head: Coordinate) {
        self.body.push_front(self.head);
        self.head = next_head;

        while self.body.len() > self.body_length {
            self.body.pop_back();
        }
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if self.direction.opposite() != new_direction {
            self.direction = new_direction;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Grid {
    width: std::num::NonZeroU16,
    height: std::num::NonZeroU16,
}

#[derive(Debug, PartialEq, Eq)]
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
    game_state: GameState,
}

impl World {
    pub const fn new(grid: Grid) -> Self {
        Self {
            snake: Snake::new(
                Coordinate::new(grid.width.get() / 2, grid.height.get() / 2),
                3,
            ),
            grid,
            game_state: GameState::Active,
        }
    }

    pub fn tick(&mut self) {
        if self.game_state == GameState::Active
            && let Some(next_head) = self.snake.next_head_position()
            && next_head.is_inside(self.grid)
        {
            self.snake.do_move(next_head);
        } else {
            self.game_state = GameState::GameOver;
        }
    }
}
