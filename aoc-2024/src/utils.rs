use core::fmt;
use itertools::Itertools;
use std::{fmt::Write, ops::Range};

/**
Stores a position in a 2D array with the format of (X, Y)
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(pub usize, pub usize);

impl From<usize> for Position {
    /// a => (a, a)
    fn from(value: usize) -> Self {
        Self(value, value)
    }
}
impl From<&[usize]> for Position {
    /// &[usize] => (a.0, a.1)
    fn from(value: &[usize]) -> Self {
        Self(value[0], value[1])
    }
}
impl From<&Vec<usize>> for Position {
    /// vec![usize] => (a.0, a.1)
    fn from(value: &Vec<usize>) -> Self {
        Self(value[0], value[1])
    }
}

#[allow(dead_code)]
impl Position {
    /**
    get the next position following that direction.
    */
    pub fn next_pos(&self, direction: &Direction) -> Self {
        match direction {
            Direction::West => Position(self.0 - 1, self.1),
            Direction::South => Position(self.0, self.1 + 1),
            Direction::East => Position(self.0 + 1, self.1),
            Direction::North => Position(self.0, self.1 - 1),
            Direction::SouthWest => Position(self.0 - 1, self.1 + 1),
            Direction::SouthEast => Position(self.0 + 1, self.1 + 1),
            Direction::NorthEast => Position(self.0 + 1, self.1 - 1),
            Direction::NorthWest => Position(self.0 - 1, self.1 - 1),
        }
    }

    /// Checks if the next position is valid
    pub fn is_next_valid(&self, direction: &Direction, grid_size: &Position) -> bool {
        match direction {
            Direction::North => self.1 >= 1,
            Direction::East => self.0 + 1 < grid_size.0,
            Direction::South => self.1 + 1 < grid_size.1,
            Direction::West => self.0 >= 1,
            Direction::NorthEast => self.0 + 1 < grid_size.0 && self.1 >= 1,
            Direction::SouthEast => self.0 + 1 < grid_size.0 && self.1 + 1 < grid_size.1,
            Direction::SouthWest => self.0 >= 1 && self.1 + 1 < grid_size.1,
            Direction::NorthWest => self.0 >= 1 && self.1 >= 1,
        }
    }

    /// check if the current position is valid.
    pub fn is_valid(&self, grid_size: &Position) -> bool {
        // pos can never be negative as that would cause errors, hence no need to check for it.
        self.0 < grid_size.0 && self.1 < grid_size.1
    }

    /// Create an empty one for other use.
    pub fn empty() -> Self {
        Self(0, 0)
    }

    pub fn from_id(id: &usize, max_size: &usize) -> Self {
        Position(id % max_size, id / max_size)
    }
    pub fn to_id(self, max_size: usize) -> usize {
        self.1 * max_size + self.0
    }

    pub const MAX: Self = Self(usize::MAX, usize::MAX);

    /// get all our neighbours. (In all Directions)
    pub fn get_neighbours(&self, grid_size: &Position, include_diagonal: bool) -> Vec<Position> {
        self.get_valid_directions(grid_size, include_diagonal)
            .iter()
            .map(|v| self.next_pos(v))
            .collect_vec()
    }

    /// get the valid directions we can move, if any.
    pub fn get_valid_directions(
        &self,
        grid_size: &Position,
        include_diagonal: bool,
    ) -> Vec<Direction> {
        let mut valid: Vec<Direction> = vec![];
        if self.0 >= 1 {
            valid.push(Direction::West);
        }
        if self.1 >= 1 {
            valid.push(Direction::North);
        }
        if self.0 + 1 < grid_size.0 {
            valid.push(Direction::East);
        }
        if self.1 + 1 < grid_size.1 {
            valid.push(Direction::South);
        }
        if include_diagonal {
            if self.0 >= 1 && self.1 + 1 < grid_size.1 {
                valid.push(Direction::SouthWest);
            }
            if self.0 + 1 < grid_size.0 && self.1 + 1 < grid_size.1 {
                valid.push(Direction::SouthEast);
            }
            if self.0 + 1 < grid_size.0 && self.1 >= 1 {
                valid.push(Direction::NorthEast);
            }
            if self.0 >= 1 && self.1 >= 1 {
                valid.push(Direction::NorthWest);
            }
        }
        valid
    }

    /// A combination of get_neighbours and get_valid_directions
    pub fn get_valid(
        &self,
        grid_size: &Position,
        include_diagonal: bool,
    ) -> Vec<(Direction, Position)> {
        self.get_valid_directions(grid_size, include_diagonal)
            .iter()
            .map(|v| (*v, self.next_pos(v)))
            .collect_vec()
    }

    /// return as a index number (used for 2d array in a 1d array situtation)
    pub fn get_as_number(&self, grid_size: &Position) -> usize {
        self.1 * grid_size.1 + self.0
    }

    // Thanks to guy_732: https://github.com/guy-732/aoc-2024/blob/01da016705b2be4cbbb199603d2cf32fcbcd0fda/src/day20.rs#L15-L34
    /// get the point distance between two points
    pub fn manhattan_distance(&self, other: &Self) -> u64 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u64
    }

    // Thanks to guy_732: https://github.com/guy-732/aoc-2024/blob/01da016705b2be4cbbb199603d2cf32fcbcd0fda/src/day20.rs#L15-L34
    /// generate an iter of all positions within a radius of max_distance
    pub fn iter_positions_within(
        &self,
        max_distance: usize,
        grid_size: Position,
    ) -> impl IntoIterator<Item = (Position, u64)> + '_ {
        let max_isize = max_distance as isize;
        // generate all points within a max_distance * 2 + 1 area
        ((self.0 as isize - max_isize)..=(self.0 as isize + max_isize))
            .flat_map(move |x_dist| {
                ((self.1 as isize - max_isize)..=(self.1 as isize + max_isize))
                    .map(move |y_dist| Position(x_dist as usize, y_dist as usize))
            })
            .map(|position| (position, self.manhattan_distance(&position)))
            // filter out those not in the area
            .filter(move |(_, dist)| (*dist as usize) <= max_distance)
            .filter(move |(pos, _)| pos.0 < grid_size.0 && pos.1 < grid_size.1)
    }

    /// loop over all positions between two positions
    pub fn iter_positions<'a>(&'a self, end: &'a Self) -> impl IntoIterator<Item = Position> + '_ {
        let (a, b) = if self < end { (self, end) } else { (end, self) };

        ((a.0 as isize)..=(b.0 as isize)).flat_map(move |x_dist| {
            ((a.1 as isize)..=(b.1 as isize))
                .map(move |y_dist| Position(x_dist as usize, y_dist as usize))
        })
    }

    /// check if itself is the corner of the grid
    pub fn is_corner(&self, grid_size: &Position) -> bool {
        (self.0 == grid_size.0 - 1 || self.0 == 0) && (self.1 == grid_size.1 - 1 || self.1 == 0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::NorthEast,
            5 => Direction::SouthEast,
            6 => Direction::SouthWest,
            7 => Direction::NorthWest,
            _ => panic!("Invalid direction"),
        }
    }
}

#[allow(dead_code)]
impl Direction {
    /// Useful for quickly going in the opposite direction
    pub fn inverse(&self) -> Self {
        match self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::NorthEast => Direction::SouthWest,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast,
            Direction::NorthWest => Direction::SouthEast,
        }
    }

    /// useful for debugging
    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn rotate(&self, diagonal: bool) -> Self {
        match self {
            Self::North => {
                if diagonal {
                    Self::NorthEast
                } else {
                    Self::East
                }
            }
            Self::East => {
                if diagonal {
                    Self::SouthEast
                } else {
                    Self::South
                }
            }
            Self::South => {
                if diagonal {
                    Self::SouthWest
                } else {
                    Self::West
                }
            }
            Self::West => {
                if diagonal {
                    Self::NorthWest
                } else {
                    Self::North
                }
            }
            Self::NorthEast => {
                if diagonal {
                    Self::East
                } else {
                    Self::SouthEast
                }
            }
            Self::SouthEast => {
                if diagonal {
                    Self::South
                } else {
                    Self::SouthWest
                }
            }
            Self::SouthWest => {
                if diagonal {
                    Self::West
                } else {
                    Self::NorthWest
                }
            }
            Self::NorthWest => {
                if diagonal {
                    Self::North
                } else {
                    Self::NorthEast
                }
            }
        }
    }

    pub fn all(diagonal: bool) -> Vec<Direction> {
        if !diagonal {
            vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
        } else {
            vec![
                Direction::North,
                Direction::NorthEast,
                Direction::East,
                Direction::SouthEast,
                Direction::South,
                Direction::SouthWest,
                Direction::West,
                Direction::NorthWest,
            ]
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
                    let mut yy = y.iter().fold(String::new(), |mut output, x| {
                        // allow each cell to debug how this wish to debug.
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

#[allow(dead_code)]
impl<T: std::clone::Clone> Grid<T> {
    /// create a new grid of set size and fill with the default argument
    pub fn new(size: &Position, default_arg: T) -> Self {
        Self(vec![vec![default_arg; size.1]; size.0])
    }

    /// create a grid from a string. Useful for loading from files.
    ///
    /// [translation] is a user-provided method to take the char and translate it into whatever the grid needs to be.
    pub fn from_str<F: Fn(&char) -> T>(s: &str, default_arg: T, translation: F) -> Self {
        let size = s.lines().count().max(s.lines().nth(0).unwrap().len());
        let mut grid = Self::new(&Position::from(size), default_arg);
        s.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, char)| {
                grid.set_cell(&Position(x, y), translation(&char));
            });
        });
        grid
    }

    /// set a specific value to a specific cell.
    ///
    /// Prefered use of [get_cell] to set data if data has already been set
    pub fn set_cell(&mut self, pos: &Position, value: T) {
        self[pos.1][pos.0] = value;
    }

    pub fn set_cells<'a>(&mut self, cells: impl IntoIterator<Item = &'a Position>, value: T) {
        cells.into_iter().for_each(|&cell| {
            self.set_cell(&cell, value.to_owned());
        });
    }

    /// set all the corners of the grid.
    pub fn set_corners(&mut self, value: T) {
        self[0][0] = value.to_owned();
        let size = self.get_size();
        self[0][size.0 - 1] = value.to_owned();
        self[size.1 - 1][0] = value.to_owned();
        self[size.1 - 1][size.0 - 1] = value.to_owned();
    }

    /// set all the cells within a range of x to y.
    pub fn set_cell_range(&mut self, start_pos: &Position, end_pos: &Position, value: T) {
        for pos in start_pos.iter_positions(end_pos) {
            self.set_cell(&pos, value.to_owned());
        }
    }

    /// same as [set_cell_range] but calls the user provided [callback] function on the old cell value before setting it to the new cell value
    pub fn get_set_cell_range<F: FnMut(&mut T) -> T>(
        &mut self,
        start_pos: &Position,
        end_pos: &Position,
        mut callback: F,
    ) {
        for pos in start_pos.iter_positions(end_pos) {
            let cell = self.get_cell(&pos);
            *cell = callback(cell);
        }
    }

    /// returns a mutable copy of a cell
    pub fn get_cell(&mut self, pos: &Position) -> &mut T {
        &mut self[pos.1][pos.0]
    }

    /// returns ownership of a cell value
    pub fn get_unmut_cell(&self, pos: &Position) -> T {
        self[pos.1][pos.0].to_owned()
    }

    pub fn get_cell_refrence(&self, pos: &Position) -> &T {
        &self[pos.1][pos.0]
    }

    /// returns the size of the grid
    pub fn get_size(&self) -> Position {
        Position(
            self.first().expect("Failed to get first row").len(),
            self.len(),
        )
    }

    pub fn find_cell<P: FnMut(&T) -> bool>(&self, predicate: P) -> Option<T> {
        self.iter_unmut_cells().into_iter().find(predicate)
    }

    pub fn cell_position<P: FnMut(T) -> bool>(&self, predicate: P) -> Option<Position> {
        self.iter_unmut_cells()
            .into_iter()
            .position(predicate)
            .map(|v| Position::from_id(&v, &self.get_size().1))
    }

    /// loop over all unmutable cell values
    pub fn iter_unmut_cells(&self) -> impl IntoIterator<Item = T> + '_ {
        let size = self.get_size();
        (0..size.1)
            .flat_map(move |y| (0..size.0).map(move |x| self.get_unmut_cell(&Position(x, y))))
    }

    /// loop over all positions in the grid
    pub fn iter_positions(&self) -> impl IntoIterator<Item = Position> + '_ {
        let size = self.get_size();
        (0..size.1).flat_map(move |y| (0..size.0).map(move |x| Position(x, y)))
    }
}

/// Contains a list and default value. Designed to be used to wrap things around
pub struct Incrementer<T> {
    list: Vec<T>,
    default: T,
}

#[allow(dead_code)]
impl<T: std::clone::Clone + std::cmp::PartialEq> Incrementer<T> {
    /// Create a new wrapper of len x
    pub fn new(default_value: T, length: usize) -> Self {
        Self {
            list: vec![default_value.clone(); length],
            default: default_value,
        }
    }

    /// Wrap the wrapper around
    /// [next] is a function that takes in the current and gives the next. If the returned result is the same as the default value, then the next one moves along
    /// Returns how many values got updated
    pub fn wrap<F: Fn(&T) -> T>(&mut self, next: F) -> usize {
        let mut index = self.len() - 1;
        loop {
            let v = next(&self[index]);
            self[index] = v;

            if index == 0 && self.default == self[index] {
                // reached end
                break self.len() - index + 1;
            }

            if self[index] != self.default {
                // no need to as we didn't loop this time
                break self.len() - index;
            }
            if index == 0 {
                // prevents from going into negatives
                break self.len() - index;
            }
            index -= 1;
        }
    }

    /// Same as [wrap] but starts at a specific index instead
    pub fn big_wrap<F: Fn(&T) -> T>(&mut self, next: F, mut index: usize) -> usize {
        for i in index..self.len() - 1 {
            self[i] = self.default.clone();
        }

        loop {
            let v = next(&self[index]);
            self[index] = v;

            if index == 0 && self.default == self[index] {
                break self.len() - index;
            }

            if self[index] != self.default {
                // no need to as we didn't loop this time
                break self.len() - index;
            }
            if index == 0 {
                // prevents from going into negatives
                break self.len() - index + 1;
            }
            index -= 1;
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}

impl<T> std::ops::Index<usize> for Incrementer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index]
    }
}

impl<T> std::ops::Index<Range<usize>> for Incrementer<T> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.list[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Incrementer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.list[index]
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Incrementer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.list)
    }
}

pub trait Length {
    // 340,282,366,920,938,463,463,374,607,431,768,211,455
    fn number_length(&self) -> u8;
}

impl Length for u8 {
    fn number_length(&self) -> u8 {
        10_u64.pow(((self + 1) as f64).log10().ceil() as u32) as u8
    }
}

impl Length for u16 {
    fn number_length(&self) -> u8 {
        10_u64.pow(((self + 1) as f64).log10().ceil() as u32) as u8
    }
}

impl Length for u32 {
    fn number_length(&self) -> u8 {
        10_u64.pow(((self + 1) as f64).log10().ceil() as u32) as u8
    }
}

impl Length for u64 {
    fn number_length(&self) -> u8 {
        10_u64.pow(((self + 1) as f64).log10().ceil() as u32) as u8
    }
}

impl Length for u128 {
    fn number_length(&self) -> u8 {
        10_u64.pow(((self + 1) as f64).log10().ceil() as u32) as u8
    }
}

impl Length for usize {
    fn number_length(&self) -> u8 {
        10_u64.pow(((self + 1) as f64).log10().ceil() as u32) as u8
    }
}

#[allow(dead_code)]
pub fn print_and_return<T: std::fmt::Debug>(v: T) -> T {
    println!("{:?}", v);
    v
}
