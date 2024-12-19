/**
 * @param {NS} ns
 * @param {String[]} board
 */
function print_board(ns, board) {
    board.forEach((line) => {
        ns.tprintf(line);
    })
}

/** @param {NS} ns */
export async function main(ns) {
    // load the data
    // acceptable auguments: ['r', 'd', 'd2', 't'];
    // 'r' = real, 'd' = deafult (part1/part2), 'd2' = default (part2 will do part1 if no part2 found), 't' = test (custom data)
    let data = ns.read(`AOC/Data/2023/13/${ns.args[0]}.txt`);
    let game_data = data.split('\n');
    // Can't remove empty spaces as we need them to work out the different boards.
    let grids = get_grids(game_data);
    let sum = 0;
    grids.forEach((grid, gid) => {
        console.log(`Scanning: ${gid} -- ${grid}`);
        // let mirror = process_grid2(grid);
        // let rotated = false;
        // if (mirror == -1) {
        //     rotated = true;
        //     mirror = process_grid_horizontal(grid);

        //     if (mirror == -1) {
        //         ns.tprintf(`ERROR: No mirror position for:`);
        //         print_board(ns, grid);
        //         ns.exit();
        //     }

        //     sum += mirror;
        // } else {
        //     sum += (mirror * 100);
        // }
        let mirror = guy(ns, grid);
        sum += mirror;
        ns.tprint(`Mirror detected at: ${mirror} on ${gid}`);
    })

    ns.tprint(`Result: ${sum}`);
}

/**
 * @param {String[]} data
 */
function get_grids(data) {
    let grids = [];
    let temp = [];
    data.forEach((line) => {
        if (line == '') {
            if (temp.length > 0) {
                grids.push(temp);
            }
            temp = [];
            return;
        }

        temp.push(line);
    })

    return grids;
}

/**
 * Assume both a and b are same length
 * @param {String} a
 * @param {String} b
 */
function get_differences(a, b) {
    let differences = [];
    for (let i = 0; i < a.length; i++) {
        if (a[i] != b[i]) {
            differences.push(i);
        }
    }
    return differences;
}

/**
 * @param {String[]} grid
 */
function process_grid(grid) {
    let stack = [];
    let mirror = -1;
    grid.forEach((line, idx) => {
        let line_check = false;
        let is_mirrored = false;
        if (mirror > 0) {
            return;
        }

        if (line == stack[stack.length - 1] || (stack.length > 0 && get_differences(line, stack[stack.length - 1]).length == 1)) {
            console.log(line, stack, stack[stack.length - 1], idx, grid.length);
            // check to make sure it is a mirror and not just a lucky dupe
            for (let i = idx; i < grid.length; i++) {
                // if we have gotten this far, we have checked all that are needed.
                if (stack.length - 1 - (i - idx) < 0) {
                    break;
                }

                if (grid[i] != stack[stack.length - 1 - (i - idx)]) {
                    perfect = false;
                    if (!line_check) {
                        line_check = true;
                        is_mirrored = true;
                        continue;
                    } else {
                        is_mirrored = false;
                        break;
                    }
                    // stack.push(line);
                    // return;
                    // break;
                }
            }
            if (is_mirrored || idx + 1 == grid.length) {
                // we hope this is mirror line
                mirror = idx;
            }
        }

        stack.push(line);
    })

    return mirror;
}

/**
 * Guy version 
 * @param {NS} ns
 * @param {String[] grid} 
 * */
function guy(ns, grid) {
    let r = process_grid2(grid);
    if (r == -1) {
        r = process_grid2_horizontal(grid);
        if (r == -1) {
            ns.tprintf('ERROR: Could not place mirror!');
            ns.exit();
        }

        r *= 100;
    }

    return r;
}

/**
 * Guy version 
 * @param {String[] grid} 
 * */
function process_grid2(grid) {
    for (let i = 1; i < grid[0].length; i++) {
        let cols_to_compare = Math.min(i, grid[0].length - i);
        let is_mirrored = false;
        let has_one_mistake = false;
        for (let j = 0; j < grid.length; j++) {
            let row = grid[j];
            let brk = false;
            for (let comp = 0; comp < cols_to_compare; comp++) {
                if (row[i - comp - 1] != row[i + comp]) {
                    if (!has_one_mistake) {
                        has_one_mistake = true;
                        is_mirrored = true;
                    } else {
                        is_mirrored = false;
                        brk = true;
                        break;
                    }
                }
            }

            if (brk) {
                break;
            }
        }
        if (is_mirrored) {
            return i;
        }
    }

    return -1;
}

/**
 * Guy version 
 * @param {String[] grid} 
 * */
function process_grid2_horizontal(grid) {
    for (let i = 1; i < grid.length; i++) {
        let rows_to_compare = Math.min(i, grid.length - i);
        let has_one_mistake = false;
        for (let comp = 0; comp < rows_to_compare; comp++) {
            let brk = false;
            for (let j = 0; j < grid[0].length; j++) {
                let r1 = grid[i + comp][j];
                let r2 = grid[i - comp - 1][j];
                if (r1 != r2) {
                    if (has_one_mistake) {
                        has_one_mistake = false; // since we break out of the loop the condition will ignore this and skip to the next
                        brk = true;
                        break;
                    } else {
                        has_one_mistake = true;
                    }
                }
            }

            if (brk) {
                break;
            }
        }
        if (has_one_mistake) {
            return i;
        }
    }

    return -1;
}

/**
 * @param {String[]} grid
 */
function process_grid_horizontal(grid) {
    let rotated = [];
    // rotates the board to work with the checker
    for (let i = 0; i < grid[0].length; i++) {
        let temp = [];
        for (let j = grid.length - 1; j >= 0; j--) {
            temp.push(grid[j][i]);
        }
        rotated.push(temp.join(''));
    }

    console.log(rotated);

    // now we just pass this into the other function
    return process_grid(rotated);
}



/**
 * This is mainly a backup
 * @param {String[]} grid
 */
function process_grid_p1(grid) {
    let stack = [];
    let mirror = -1;
    grid.forEach((line, idx) => {
        if (mirror > 0) {
            return;
        }

        if (line == stack[stack.length - 1]) {
            // check to make sure it is a mirror and not just a lucky dupe
            for (let i = idx; i < grid.length; i++) {
                // if we have gotten this far, we have checked all that are needed.
                if (stack.length - 1 - (i - idx) < 0) {
                    break;
                }

                if (grid[i] != stack[stack.length - 1 - (i - idx)]) {
                    stack.push(line);
                    return;
                    // break;
                }
            }

            mirror = idx;
        }

        stack.push(line);
    })

    return mirror;
}