use aoc_runner_derive::{aoc, aoc_generator};

// Cell struct to keep track of every cell.
#[derive(Debug, Clone, Copy)]
struct Cell {
    empty: bool,    // Tells if the cell is empty of not
    id: u32, // Tells the id of the cell if it has one. Cell id can be 0 hence the use of empty
    size: u32, // Tells the size of the size
    overflow: bool, // Tells if the cell should only allow cells which are smaller or same size in it
}

impl Cell {
    /**
     * Fills up this cell with another cell
     * Self -> This cell
     * Other -> The cell to put into this cell
     * Returns -> A new cell with the left over space from putting other into this cell.
     */
    fn fill_cell(&mut self, other: &mut Cell) -> Cell {
        // default cell object, designed for use when we need to return the left over.
        let mut cell = Cell {
            empty: true,
            id: 0,
            size: 0,
            overflow: self.overflow,
        };

        // Checks to see if we are empty and we are overflowing
        if self.empty && self.overflow {
            // Checks to see if the other cell can fit in us.
            if self.size >= other.size {
                // set our id to the one comming into us and make us not empty
                self.id = other.id;
                self.empty = false;
                other.empty = true;

                // break our cell up into two
                let overflow = self.size - other.size;
                self.size = other.size;
                cell.size = overflow;

                return cell;
            }
            return cell;
        }

        // if we aren't empty, then there is no point in looking at filling us up. (we have data, we aren't mean enough to override it...)
        if !self.empty {
            return cell;
        }

        self.empty = false;
        self.id = other.id;

        if self.size < other.size {
            // If we don't have enough space, fill us up by reducing the amount from the other cell
            let unused = other.size - self.size;
            other.size = unused;
            return cell;
        }

        if self.size > other.size {
            // If we have more than enough space, fill us up with the other cell size and break off the new cell.
            cell.size = self.size - other.size;
            self.size = other.size;
            // Other cell is emptied
            other.size = 0;
            other.empty = true;
            return cell;
        }

        // if we have the same size, empty the other cell
        if self.size == other.size {
            other.size = 0;
            other.empty = true;
        }

        cell
    }

    // Checks to see if the other cell can fit in our cell
    fn can_fit(&self, other: Cell) -> bool {
        // println!(
        //     "Testing to see if {:?} can fit in {:?} (Result: {:?})",
        //     other,
        //     self,
        //     self.size >= other.size
        // );
        self.size >= other.size
    }

    // Checks to see if we have no size (aka no space to put anything)
    fn has_no_size(&self) -> bool {
        self.size == 0
    }

    // Calculates the checksum of this cell
    fn calculate_checksum(&self, position: u32) -> (u32, u64) {
        if self.empty {
            // Its empty here, hence no data, hence always a 0
            return (position + self.size, 0);
        }

        let mut checksum = 0;
        for index in 0..self.size {
            checksum += self.id * (position + index);
        }
        (position + self.size, checksum as u64)
    }

    // Debug purpose: Converts our cell to a string representation of what it would actually look like. Doesn't do well with numbers > 9
    fn real_repersentation(&self) -> String {
        let vec: Vec<String> = vec![
            if self.empty {
                '.'.to_string()
            } else {
                self.id.to_string()
            };
            self.size as usize
        ];
        vec.join("")
    }
}

// Converts the input into cells
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Cell> {
    let mut cells: Vec<Cell> = vec![];
    let mut is_empty: bool = false;
    let mut id: u32 = 0;

    for char in input.chars() {
        // for all characters make an empty cell
        let cell = Cell {
            empty: is_empty,
            id: if is_empty { 0 } else { id },
            size: char.to_digit(10).expect("Failed to parse digit in input."),
            overflow: false,
        };
        cells.push(cell);
        if !is_empty {
            // give each cell that isn't empty their own id.
            id += 1;
        }
        is_empty = !is_empty;
    }
    cells
}

// Returns the cell at the back of the list that isn't empty.
fn get_back_cell(cell_list: &Vec<Cell>, mut back_list_index: usize) -> (usize, Cell) {
    loop {
        // If we empty out of the list just break, we aren't going to find cells in negative territory.
        if back_list_index >= cell_list.len() {
            break (
                0,
                Cell {
                    empty: true,
                    id: 0,
                    size: 0,
                    overflow: false,
                },
            );
        }

        let back_cell = cell_list[cell_list.len() - back_list_index];
        if back_cell.has_no_size() {
            back_list_index += 1;
            continue;
        }
        if back_cell.empty {
            back_list_index += 1;
            continue;
        }
        break (back_list_index, back_cell);
    }
}

// The same as [get_back_cell] but only returns the cell if it can fit in the current cell.
fn get_fit_back_cell(cell_list: &Vec<Cell>, cell: usize) -> (usize, Cell) {
    let current_cell = cell_list[cell];
    let mut back_index = 1;
    loop {
        let back = cell_list[cell_list.len() - back_index];
        if !back.empty {
            if current_cell.can_fit(back) {
                break (back_index, back);
            }
        }

        back_index += 1;
        if back_index > cell_list.len() - 1 {
            break (
                0,
                Cell {
                    empty: true,
                    id: 0,
                    size: 0,
                    overflow: true,
                },
            );
        }
    }
}

// Recreate the file system by using the old file system and hence filling up the spaces which are empty.
fn fill_up_cells(cells: &Vec<Cell>) -> Vec<Cell> {
    let mut cell_list = cells.clone();
    let mut new_list: Vec<Cell> = vec![];
    let mut back_list_index = 1;

    for cell in 0..cell_list.len() {
        // println!("{:?}", cell_list);
        // print_real(&new_list);
        // println!("{:?} {:?}", cell, back_list_index);
        let front_cell = cell_list[cell];
        // No place to fill up if there isn't any size
        if front_cell.has_no_size() {
            continue;
        }

        // The front cell already has data, we don't overwrite.
        if !front_cell.empty {
            new_list.push(front_cell);
            cell_list[cell].empty = true;
            continue;
        }

        // Depending on thee part, get a cell which can fill or the most back cell in general
        let mut back_cell;
        if !front_cell.overflow {
            (back_list_index, back_cell) = get_back_cell(&cell_list, back_list_index);
            if back_cell.empty {
                // Part 1: If we can't find any we probably met up in the middle.
                break;
            }
        } else {
            (back_list_index, back_cell) = get_fit_back_cell(&cell_list, cell);
            if back_cell.empty {
                // Part 2: Just because we couldn't find any doesn't mean we are done yet.
                new_list.push(front_cell);
                continue;
            }
        }
        // println!("BLI: {:?} CELL: {:?}", back_list_index, cell);

        // Move the back cell into the front cell.
        let mut clone_front = front_cell.clone();
        // println!("{:?} {:?}", clone_front, back_cell);
        let extra = clone_front.fill_cell(&mut back_cell);
        // println!("{:?} {:?} {:?}", clone_front, back_cell, extra);

        let cell_len = cell_list.len();
        cell_list[cell_len - back_list_index] = back_cell; // replace the back cell with the new back cell, otherwise issues will happen.

        new_list.push(clone_front);
        if !extra.has_no_size() {
            cell_list.insert(cell + 1, extra);
            // new_list.push(extra);
        }
    }

    new_list
}

fn calculate_checksum(cells: &Vec<Cell>) -> u64 {
    let mut checksum = 0;
    let mut position = 0;
    for cell in cells.iter() {
        let (r_pos, r_check) = cell.calculate_checksum(position);
        position = r_pos;
        checksum += r_check;
    }
    checksum
}

// Debug
#[allow(dead_code)]
fn print_real(cells: &Vec<Cell>) {
    for cell in cells.iter() {
        print!("{}", cell.real_repersentation());
    }
    println!();
}

#[aoc(day9, part1)]
fn part1(input: &Vec<Cell>) -> u64 {
    // println!("{:?}", input);
    // println!();

    let cells = fill_up_cells(input);
    // println!();
    // println!("{:?}", cells);
    // print_real(&cells);
    calculate_checksum(&cells)
}

#[aoc(day9, part2)]
fn part2(input: &Vec<Cell>) -> u64 {
    let mut cells = input.clone();
    cells.iter_mut().for_each(|cell| cell.overflow = true);

    // print_real(&cells);
    let result_cells = fill_up_cells(&cells);
    // print_real(&result_cells);
    calculate_checksum(&result_cells)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "2333133121414131402";
    const EXAMPLE_2: &str = "12345";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 1928);
    }
    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(EXAMPLE_2)), 60);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 2858);
    }
}
