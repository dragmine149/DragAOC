use core::fmt;
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(pub usize, pub usize);

impl From<usize> for Position {
    fn from(value: usize) -> Self {
        Self(value, value)
    }
}
impl From<&[usize]> for Position {
    fn from(value: &[usize]) -> Self {
        Self(value[0], value[1])
    }
}
impl From<&Vec<usize>> for Position {
    fn from(value: &Vec<usize>) -> Self {
        Self(value[0], value[1])
    }
}

impl Position {
    #[allow(dead_code)]
    pub fn next_pos(&self, direction: &Direction) -> Self {
        match direction {
            Direction::North => Position(self.0 - 1, self.1),
            Direction::East => Position(self.0, self.1 + 1),
            Direction::South => Position(self.0 + 1, self.1),
            Direction::West => Position(self.0, self.1 - 1),
        }
    }

    #[allow(dead_code)]
    pub fn is_valid(&self, grid_size: &Position) -> bool {
        // pos can never be negative as that cause some rust errors.
        self.0 < grid_size.0 && self.1 < grid_size.1
    }

    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self(0, 0)
    }
    #[allow(dead_code)]
    pub fn get_valid_positions(&self, grid_size: &Position) -> Vec<Position> {
        let mut valid: Vec<Position> = vec![];
        if self.0 >= 1 {
            valid.push(Position(self.0 - 1, self.1));
        }
        if self.1 >= 1 {
            valid.push(Position(self.0, self.1 - 1));
        }
        if self.0 < grid_size.1 {
            valid.push(Position(self.0 + 1, self.1));
        }
        if self.1 < grid_size.0 {
            valid.push(Position(self.0, self.1 + 1));
        }
        valid
    }

    #[allow(dead_code)]
    pub fn get_valid_directions(&self, grid_size: &Position) -> Vec<Direction> {
        let mut valid: Vec<Direction> = vec![];
        if self.0 >= 1 {
            valid.push(Direction::North);
        }
        if self.1 >= 1 {
            valid.push(Direction::West);
        }
        if self.0 < grid_size.1 {
            valid.push(Direction::South);
        }
        if self.1 < grid_size.0 {
            valid.push(Direction::East);
        }
        valid
    }

    #[allow(dead_code)]
    pub fn get_valid(&self, grid_size: &Position) -> Vec<(Direction, Position)> {
        let mut valid: Vec<(Direction, Position)> = vec![];
        if self.0 >= 1 {
            valid.push((Direction::North, Position(self.0 - 1, self.1)));
        }
        if self.1 >= 1 {
            valid.push((Direction::West, Position(self.0, self.1 - 1)));
        }
        if self.0 < grid_size.1 {
            valid.push((Direction::South, Position(self.0 + 1, self.1)));
        }
        if self.1 < grid_size.0 {
            valid.push((Direction::East, Position(self.0, self.1 + 1)));
        }
        valid
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    // Useful for quickly going in the opposite direction
    #[allow(dead_code)]
    pub fn inverse(&self) -> Self {
        match self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
        }
    }

    #[allow(dead_code)]
    pub fn index(&self) -> usize {
        *self as usize
    }
}

pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T: fmt::Debug> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .map(|y| {
                    let mut yy = y.iter().fold(String::new(), |mut output, x| {
                        if f.alternate() {
                            let _ = write!(output, "{:#?}", x);
                            output
                        } else {
                            let _ = write!(output, "{:?}", x);
                            output
                        }
                    });
                    yy.push('\n');
                    yy
                })
                .collect::<String>()
        )
    }
}

impl<T: std::clone::Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        let mut clone: Vec<Vec<T>> = vec![];
        for row in self.iter() {
            clone.push(row.to_vec());
        }

        Self(clone)
    }
}

impl<T> std::ops::Deref for Grid<T> {
    type Target = [Vec<T>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: std::clone::Clone> Grid<T> {
    #[allow(dead_code)]
    pub fn new(size: Position, default_arg: T) -> Self {
        Self(vec![vec![default_arg; size.1]; size.0])
    }

    #[allow(dead_code)]
    pub fn set_cell(&mut self, pos: Position, value: T) {
        self[pos.1][pos.0] = value;
    }

    #[allow(dead_code)]
    pub fn get_cell(&mut self, pos: Position) -> &mut T {
        &mut self[pos.1][pos.0]
    }

    #[allow(dead_code)]
    pub fn get_unmut_cell(&self, pos: Position) -> T {
        self[pos.1][pos.0].to_owned()
    }

    #[allow(dead_code)]
    pub fn get_size(&self) -> Position {
        Position(
            self.first().expect("Failed to get first row").len(),
            self.len(),
        )
    }
}
