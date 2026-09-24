use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    x: u16,
    y: u16,
}

impl Coordinate {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn get_coordinate(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Snake {
    body: VecDeque<Coordinate>,
    body_length: usize,
    direction: Direction,
}

impl Snake {
    pub fn new(start_size: u16) -> Self {
        assert!(start_size >= 1);

        let mut snake = Self {
            body: VecDeque::new(),
            body_length: start_size as usize,
            direction: Direction::Up,
        };
        snake.body.push_front(Coordinate::new());
        for _ in 1..start_size {
            snake.do_move()
        }
        snake
    }

    pub fn get_head_position(&self) -> &Coordinate {
        &self.body[0]
    }

    pub fn do_move(&mut self) -> () {
        let head_position = self.get_head_position();
        self.body.push_front(match self.direction {
            Direction::Up => Coordinate {
                x: head_position.x,
                y: head_position.y + 1,
            },
            Direction::Down => Coordinate {
                x: head_position.x,
                y: head_position.y - 1,
            },
            Direction::Right => Coordinate {
                x: head_position.x + 1,
                y: head_position.y,
            },
            Direction::Left => Coordinate {
                x: head_position.x - 1,
                y: head_position.y,
            },
        });

        while self.body.len() > self.body_length {
            self.body.pop_back();
        }
    }

    pub fn change_direction(&mut self, new_direction: Direction) -> () {
        if self.direction.opposite() != new_direction {
            self.direction = new_direction;
        }
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }
}
