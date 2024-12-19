// const cycles = 1_000_000_000;
// const cycles = 3;
// const cycles = 100_000;
// const cycles = 1_000_000;
// const cycles = 10_000_000;
// const cycles = 100_000_000;
const cycles = 1_000;

/** @param {NS} ns */
export async function main(ns) {
    // load the data
    // acceptable auguments: ['r', 'd', 'd2', 't'];
    // 'r' = real, 'd' = deafult (part1/part2), 'd2' = default (part2 will do part1 if no part2 found), 't' = test (custom data)
    let data = ns.read(`AOC/Data/2023/14/${ns.args[0]}.txt`);
    let game_data = data.split('\n');
    // remove empty entries from the data. (line has to be completly blank)
    game_data = game_data.filter((v) => v != '');
    game_data.forEach((line, idx) => {
        game_data[idx] = [...line];
    })
    print_board(ns, game_data);

    let time = performance.now();
    let result = game_data;

    for (let cyk = 0; cyk < cycles; cyk++) {
        // ns.tprint('north');
        result = collaspe_north(result);
        // print_board(ns, result);
        // ns.tprint('west');
        result = collapse_west(result);
        // print_board(ns, result);
        // ns.tprint('south');
        result = collaspe_south(result);
        // print_board(ns, result);
        // ns.tprint('east');
        result = collapse_east(result);
        // print_board(ns, result);

        // if (compare_grids(result, cycleStart)) {
        //     ns.tprint(`Loop breaked out of at ${cyk} due to the same data being found! (nothing really changing)`);
        //     break;
        // }

        if (cyk % 10_000_000 == 0) {
            console.log(`${cyk}: ${performance.now() - time}`);
        }
    }
    let end = performance.now();
    // collaspe_north(game_data);
    // ns.tprint('After processing...');
    ns.tprintf('\n');
    ns.tprintf('\n');
    ns.tprint('Final data');
    print_board(ns, result);


    let score = 0;
    result.forEach((line, idx) => {
        score += get_row_score(line, result.length - idx);
    })

    ns.tprint(`Finial score: ${score}`);
    ns.tprint(`Time taken: ${end - time}ms`);
}

/**
 * @param {NS} ns
 * @param {String[][]} lines
 */
function print_board(ns, lines) {
    lines.forEach((line) => {
        let pLine = '';
        line.forEach((char) => {
            pLine += char;
        })
        ns.tprintf(pLine);
    })
}

/**
 * @param {String[][]} grid
 */
function collaspe_north(grid) {
    for (let rockIndex = 0; rockIndex < grid[0].length; rockIndex++) {
        let free = [];
        for (let rowIdx = 0; rowIdx < grid.length; rowIdx++) {
            switch (grid[rowIdx][rockIndex]) {
                case '.':
                    free.push(rowIdx);
                    break;
                case '#':
                    free = [];
                    break;
                case 'O':
                    let new_pos = free.splice(0, 1)[0];
                    if (new_pos == undefined) {
                        // no where to move, so we assume we haven't found one and it is compressed
                        continue;
                    }

                    grid[new_pos][rockIndex] = 'O';
                    grid[rowIdx][rockIndex] = '.'

                    // grid[new_pos] = replaceAt(grid[new_pos], rockIndex, 'O');
                    // grid[rowIdx] = replaceAt(grid[rowIdx], rockIndex, '.');
                    free.push(rowIdx);
                    break;
            }
        }
    }

    return grid;
}

/**
 * @param {String[][]} grid
 */
function collapse_west(grid) {
    // grid.forEach((lineArr, rowIdx) => {
    for (let rowIdx = 0; rowIdx < grid.length; rowIdx++) {
        // let lineArr = to_arr(line);
        let lineArr = grid[rowIdx];
        let free = [];
        for (let rockIndex = 0; rockIndex < lineArr.length; rockIndex++) {
            // console.log(`line: ${lineArr}`);
            let char = lineArr[rockIndex];
            // console.log(char, free);
            switch (char) {
                case 'O':
                    let new_pos = free.splice(0, 1)[0];
                    if (new_pos == undefined) {
                        // we don't have to move it.
                        continue;
                    }
                    // console.log(`Old idx: ${rowIdx}; new Idx: ${new_pos}`);

                    grid[rowIdx][new_pos] = 'O';
                    grid[rowIdx][rockIndex] = '.';

                    // grid[rowIdx] = replaceAt(grid[rowIdx], new_pos, 'O');
                    // grid[rowIdx] = replaceAt(grid[rowIdx], rockIndex, '.');
                    lineArr[new_pos] = 'O';
                    lineArr[rockIndex] = '.';
                    free.push(rockIndex);
                    // console.log(`updated line: ${lineArr}`);
                    break;
                case '.':
                    free.push(rockIndex);
                    break;
                case '#':
                    free = [];
                    break;
            }
        }
    }

    return grid;
}

/**
 * @param {String[][]} grid
 */
function collaspe_south(grid) {
    for (let rockIndex = 0; rockIndex < grid[0].length; rockIndex++) {
        let free = [];
        for (let rowIdx = grid.length - 1; rowIdx >= 0; rowIdx--) {
            switch (grid[rowIdx][rockIndex]) {
                case '.':
                    free.push(rowIdx);
                    break;
                case '#':
                    free = [];
                    break;
                case 'O':
                    let new_pos = free.splice(0, 1)[0];
                    if (new_pos == undefined) {
                        // no where to move, so we assume we haven't found one and it is compressed
                        continue;
                    }

                    // grid[new_pos] = replaceAt(grid[new_pos], rockIndex, 'O');
                    // grid[rowIdx] = replaceAt(grid[rowIdx], rockIndex, '.');
                    grid[new_pos][rockIndex] = 'O';
                    grid[rowIdx][rockIndex] = '.';
                    free.push(rowIdx);
                    break;
            }
        }
    }

    return grid;
}

/**
 * @param {String[][]} grid
 */
function collapse_east(grid) {
    // grid.forEach((lineArr, rowIdx) => {
    for (let rowIdx = 0; rowIdx < grid.length; rowIdx++) {
        let lineArr = grid[rowIdx];
        // let lineArr = to_arr(line);
        let free = [];
        for (let rockIndex = lineArr.length - 1; rockIndex >= 0; rockIndex--) {
            switch (lineArr[rockIndex]) {
                case '.':
                    free.push(rockIndex);
                    break;
                case '#':
                    free = [];
                    break;
                case 'O':
                    let new_pos = free.splice(0, 1)[0];
                    if (new_pos == undefined) {
                        continue;
                    }

                    // grid[rowIdx] = replaceAt(grid[rowIdx], rockIndex, '.');
                    // grid[rowIdx] = replaceAt(grid[rowIdx], new_pos, 'O');
                    grid[rowIdx][rockIndex] = '.';
                    grid[rowIdx][new_pos] = 'O';
                    // lineArr[new_pos] = 'O';
                    // lineArr[rockIndex] = '.';
                    free.push(rockIndex);
                    break;
            }
        }
    }

    return grid;
}

/**
 * @param {String[]} row
 * @param {Number} multi
 */
function get_row_score(row, multi) {
    let sum = 0
    row.forEach((char) => {
        if (char == 'O') {
            sum += multi;
        }
    })

    return sum;
}