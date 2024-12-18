use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Position(pub usize, pub usize);

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
    pub fn from(a: &[usize]) -> Self {
        Self(a[0], a[1])
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
}

pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T: fmt::Debug> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .map(|y| {
                    let mut yy = y.iter().map(|x| format!("{:?}", x)).collect::<String>();
                    yy.push('\n');
                    yy
                })
                .collect::<String>()
        )
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
}
