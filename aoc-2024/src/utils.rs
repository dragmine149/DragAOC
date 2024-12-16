// use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn next_pos(&self, direction: &Direction) -> Self {
        match direction {
            Direction::North => Position(self.0 - 1, self.1),
            Direction::East => Position(self.0, self.1 + 1),
            Direction::South => Position(self.0 + 1, self.1),
            Direction::West => Position(self.0, self.1 - 1),
        }
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
    pub fn inverse(&self) -> Self {
        match self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
        }
    }
}

// pub type Grid = Vec<Vec<_>>;
// impl fmt::Debug for Grid {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             self.iter()
//                 .map(|y| {
//                     let yy = y.iter().map(|x| x.fmt(f)).collect::<String>();
//                     yy.push('\n');
//                     yy
//                 })
//                 .collect::<String>()
//         )
//     }
// }
