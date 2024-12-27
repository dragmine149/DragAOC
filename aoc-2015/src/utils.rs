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
