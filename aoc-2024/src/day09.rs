use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
struct Cell {
    empty: bool,
    id: u32,
    size: u32,
    overflow: bool,
}

impl Cell {
    fn fill_cell(&mut self, other: &mut Cell) -> Cell {
        let mut cell = Cell {
            empty: true,
            id: 0,
            size: 0,
            overflow: self.overflow,
        };

        if self.empty && self.overflow {
            if self.size >= other.size {
                self.id = other.id;
                self.empty = false;
                other.empty = true;

                let overflow = self.size - other.size;
                self.size = other.size;
                cell.size = overflow;

                return cell;
            }
            return cell;
        }

        if !self.empty {
            return cell;
        }

        self.empty = false;
        self.id = other.id;

        if self.size < other.size {
            let unused = other.size - self.size;
            other.size = unused;
            return cell;
        }

        if self.size > other.size {
            cell.size = self.size - other.size;
            self.size = other.size;
            other.size = 0;
            other.empty = true;
            return cell;
        }

        if self.size == other.size {
            other.size = 0;
            other.empty = true;
        }

        cell
    }

    fn can_fit(&self, other: Cell) -> bool {
        // println!(
        //     "Testing to see if {:?} can fit in {:?} (Result: {:?})",
        //     other,
        //     self,
        //     self.size >= other.size
        // );
        self.size >= other.size
    }

    fn has_no_size(&self) -> bool {
        self.size == 0
    }

    fn calculate_checksum(&self, position: u32) -> (u32, u64) {
        if self.empty {
            return (position + self.size, 0);
        }

        let mut checksum = 0;
        for index in 0..self.size {
            checksum += self.id * (position + index);
        }
        (position + self.size, checksum as u64)
    }

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

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Cell> {
    let mut cells: Vec<Cell> = vec![];
    let mut is_empty: bool = false;
    let mut id: u32 = 0;

    for char in input.chars() {
        let cell = Cell {
            empty: is_empty,
            id: if is_empty { 0 } else { id },
            size: char.to_digit(10).expect("Failed to parse digit in input."),
            overflow: false,
        };
        cells.push(cell);
        if !is_empty {
            id += 1;
        }
        is_empty = !is_empty;
    }
    cells
}

fn get_back_cell(cell_list: &Vec<Cell>, mut back_list_index: usize) -> (usize, Cell) {
    loop {
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

fn fill_up_cells(cells: &Vec<Cell>) -> Vec<Cell> {
    let mut cell_list = cells.clone();
    let mut new_list: Vec<Cell> = vec![];
    let mut back_list_index = 1;

    for cell in 0..cell_list.len() {
        // println!("{:?}", cell_list);
        // print_real(&new_list);
        // println!("{:?} {:?}", cell, back_list_index);
        let front_cell = cell_list[cell];
        if front_cell.has_no_size() {
            continue;
        }

        if !front_cell.empty {
            new_list.push(front_cell);
            cell_list[cell].empty = true;
            continue;
        }

        let mut back_cell;
        if !front_cell.overflow {
            (back_list_index, back_cell) = get_back_cell(&cell_list, back_list_index);
            if back_cell.empty {
                break;
            }
        } else {
            (back_list_index, back_cell) = get_fit_back_cell(&cell_list, cell);
            if back_cell.empty {
                new_list.push(front_cell);
                continue;
            }
        }
        // println!("BLI: {:?} CELL: {:?}", back_list_index, cell);

        let mut clone_front = front_cell.clone();
        // println!("{:?} {:?}", clone_front, back_cell);
        let extra = clone_front.fill_cell(&mut back_cell);
        // println!("{:?} {:?} {:?}", clone_front, back_cell, extra);

        let cell_len = cell_list.len();
        cell_list[cell_len - back_list_index] = back_cell;

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

    print_real(&cells);
    let result_cells = fill_up_cells(&cells);
    print_real(&result_cells);
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
