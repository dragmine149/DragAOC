/**
 * @param {NS} ns
 * @param {String[]} board
 * */
function printBoard(ns, board) {
    board.forEach((line) => {
        ns.tprintf(line);
    })
}

/** @param {NS} ns */
export async function main(ns) {
    // load the data
    // acceptable auguments: ['r', 'd', 'd2', 't'];
    // 'r' = real, 'd' = deafult (part1/part2), 'd2' = default (part2 will do part1 if no part2 found), 't' = test (custom data)
    let data = ns.read(`AOC/Data/2023/11/${ns.args[0]}.txt`);
    let game_data = data.split('\n');
    // remove empty entries from the data. (line has to be completly blank)
    game_data = game_data.filter((v) => v != '');
    printBoard(ns, game_data);

    let expanded = get_expanded(game_data);
    ns.tprint(expanded);

    // let galaxies = double_due_to_time(game_data, expanded.r, expanded.c);
    // printBoard(ns, galaxies);

    let galaxies_number = get_galaxies(game_data, expanded)

    // let info = replace_galaxies_with_numbers(galaxies);
    // let numbers = info.d;
    let pairs = galaxies_pair_sum(galaxies_number.size);
    ns.tprint(pairs);
    // printBoard(ns, numbers);

    let result = work_out_pair_distance(galaxies_number);
    ns.tprint(`Result: ${result}`);
}

const space_warp = 1_000_000;
// const space_warp = 2;

/**
 * @param {String[]} data
 * @param {{'c':Number[], 'r':Number[]}} expanded
 */
function get_galaxies(data, expanded) {
    let count = new Map();
    let offset = 0;
    data.forEach((line, didx) => {
        let coffset = 0;
        to_arr(line).forEach((char, idx) => {
            if (char == '#') {
                count.set(count.size + 1, [coffset, offset]);
            }
            coffset += expanded.c.includes(idx) ? space_warp : 1;
        })

        offset += expanded.r.includes(didx) ? space_warp : 1;
    })

    return count;
}

/** @param {Number} galaxies */
function galaxies_pair_sum(galaxies) {
    if (galaxies == 0) {
        return 0;
    }
    return (galaxies - 1) + galaxies_pair_sum(galaxies - 1);
}

/** @param {String} str */
function to_arr(str) {
    return [...str];
}

/**
 * @param {String[]} data
 */
function get_expanded(data) {
    // get row numbers
    /** @type {number[]} */
    let rows = [];
    
    data.forEach((line, idx) => {
        let count_row = true;
        to_arr(line).forEach((char) => {
            if (char != '.') {
                count_row = false;
                return;
            }
        })

        if (count_row) {
            rows.push(idx);
        }
    })

    // get column numbers
    let columns = [];

    for (let col = 0; col < data.length; col++) {
        let count_col = true;
        for (let row = 0; row < data.length; row++) {
            if (data[row][col] != '.') {
                count_col = false;
                break;
            }
        }

        if (count_col) {
            columns.push(col);
        }
    }

    return {'r': rows, 'c': columns};
}

/**
 * @param {String} str
 * @param {Number} pos
 * @param {String} char
 */
function insertAt(str, pos, char) {
    let oar = to_arr(str);
    let ar = oar.slice(0, pos + 1);
    ar.push(char);
    ar = ar.concat(oar.slice(pos + 1));
    return ar.join('');
}
/**
 * @param {Map} galaxies
 */
function work_out_pair_distance(galaxies) {
    let total_path_length = 0;

    for (let i = 1; i <= galaxies.size; i++) {
        for (let j = galaxies.size; j > 0; j--) {
            if (j <= i) {
                continue;
            }

            let h = work_out_height(galaxies.get(i), galaxies.get(j));
            let w = work_out_width(galaxies.get(i), galaxies.get(j));

            total_path_length += (h + w);
        }
    }

    return total_path_length;
}

/**
 * @param {Number[]} coordA
 * @param {Number[]} coordB
 */
function work_out_height(coordA, coordB) {
    return Math.abs(coordA[0] - coordB[0])
}

/**
 * @param {Number[]} coordA
 * @param {Number[]} coordB
 */
function work_out_width(coordA, coordB) {
    return Math.abs(coordA[1] - coordB[1])
}