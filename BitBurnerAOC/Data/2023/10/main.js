/**
 * @param {NS} ns
 * @param {String} board
 */
function print_board(ns, lines) {
    lines.forEach((line) => {
        ns.tprintf(line);
    })
}

const DIRECTION = Object.freeze({
    UP: 'UP',
    LEFT: 'LEFT',
    RIGHT: 'RIGHT',
    DOWN: 'DOWN',
    UNKNOWN: 'UNKNOWN'
})

/**
 * @param {String} str
 * @param {Number} pos
 * @param {String} char
 */
function replaceAt(str, pos, char) {
    return str.substring(0, pos) + char + str.substring(pos + char.length);
}

/** @param {NS} ns */
export async function main(ns) {
    // load the data
    // acceptable auguments: ['r', 'd', 'd2', 't'];
    // 'r' = real, 'd' = deafult (part1/part2), 'd2' = default (part2 will do part1 if no part2 found), 't' = test (custom data)
    let data = ns.read(`AOC/Data/2023/10/${ns.args[0]}.txt`);
    data = data.replaceAll('|', '║');
    data = data.replaceAll('-', '═');
    data = data.replaceAll('L', '╚');
    data = data.replaceAll('J', '╝');
    data = data.replaceAll('7', '╗');
    data = data.replaceAll('F', '╔');
    let game_data = data.split('\n');
    // remove empty entries from the data. (line has to be completly blank)
    game_data = game_data.filter((v) => v != '');
    print_board(ns, game_data);

    let new_board = [];
    let line = '.'.repeat(game_data[0].length);
    for (let col = 0; col < game_data[0].length; col++) {
        new_board.push(line);
    }

    // Find start
    let start = data.replaceAll('\n', '').indexOf('S');
    let startPos = [(start % game_data[0].length), Math.floor(start / game_data.length)];
    let pos = startPos;
    let prevPos = DIRECTION.DOWN;
    let currentCell = '.';

    // Find possible paths
    // limit of 4 for now
    let count = 0;
    while (currentCell != 'S') {
        let r = find_paths_from_cell(ns, game_data, pos, prevPos);

        new_board[pos[1]] = replaceAt(new_board[pos[1]], pos[0], get_cell(game_data, pos));

        prevPos = r[1];
        switch (r[0]) {
            case DIRECTION.UP:
                pos[1] -= 1;
                break;
            case DIRECTION.DOWN:
                pos[1] += 1;
                break;
            case DIRECTION.LEFT:
                pos[0] -= 1;
                break;
            case DIRECTION.RIGHT:
                pos[0] += 1;
                break;
        }

        currentCell = get_cell(game_data, pos);
        count += 1;

        // it should not reach 100 mil
        if (count == (game_data[0].length * game_data.length)) {
            currentCell = 'S';
            ns.tprintf("ERROR: FAILED TO FIND LOOP!");
            ns.exit();
        }
    }

    ns.tprintf(`Count: ${count / 2}`);
    print_board(ns, new_board);

    ns.tprintf(`Inside Count: ${find_inside(ns, new_board)}`);
}

/**
 * @param {string[]} data
 * @param {number[]} pos
 */
function get_cell(data, pos) {
    return data[pos[1]][pos[0]];
}

/**
 * @param {NS} ns
 * @param {string[]} data
 * @param {number[]} pos
 * @param {DIRECTION} fromPos
 */
function find_paths_from_cell(ns, data, pos, fromPos) {
    // thankfully we don't have to do diagonal.
    let x = pos[0];
    let y = pos[1];
    // get the cell info
    let cell = get_cell(data, pos);

    switch (cell) {
        case "║":
            if (fromPos == DIRECTION.DOWN) {
                return [DIRECTION.UP, DIRECTION.DOWN];
            }
            return [DIRECTION.DOWN, DIRECTION.UP];
        case "═":
            if (fromPos == DIRECTION.LEFT) {
                return [DIRECTION.RIGHT, DIRECTION.LEFT];
            }
            return [DIRECTION.LEFT, DIRECTION.RIGHT];
        case '╚':
            if (fromPos == DIRECTION.UP) {
                return [DIRECTION.RIGHT, DIRECTION.LEFT];
            }
            return [DIRECTION.UP, DIRECTION.DOWN];
        case '╝':
            if (fromPos == DIRECTION.UP) {
                return [DIRECTION.LEFT, DIRECTION.RIGHT];
            }
            return [DIRECTION.UP, DIRECTION.DOWN];
        case '╗':
            if (fromPos == DIRECTION.LEFT) {
                return [DIRECTION.DOWN, DIRECTION.UP];
            }
            return [DIRECTION.LEFT, DIRECTION.RIGHT];
        case '╔':
            if (fromPos == DIRECTION.RIGHT) {
                return [DIRECTION.DOWN, DIRECTION.UP];
            }
            return [DIRECTION.RIGHT, DIRECTION.LEFT];
        case 'S':
            if (y - 1 >= 0) {
                let up = get_cell(data, [x, y - 1]);
                if (up == '╔' || up == '╗' || up == '║')
                    return [DIRECTION.UP, DIRECTION.DOWN];
            }
            
            if (x + 1 < data[0].length) {
                let right = get_cell(data, [x + 1, y]);
                if (right == '═' || right == '╝' || right == '╗')
                    return [DIRECTION.RIGHT, DIRECTION.LEFT];
            }
            
            if (y + 1 < data.length) {
                let down = get_cell(data, [x, y + 1]);
                if (down == '║' || down == '╚' || down == '╝')
                    return [DIRECTION.DOWN, DIRECTION.UP];
            }
            
            if (x - 1 <= 0) {
                let left = get_cell(data, [x - 1, y]);
                if (left == '═' || left == '╝' || left == '╗')
                    return [DIRECTION.LEFT, DIRECTION.RIGHT];
            }

            return [DIRECTION.UNKNOWN, DIRECTION.UNKNOWN];
        default:
            ns.tprintf('WARNING: UNKOWN LOCATION!');
            ns.tprintf(`WARNING: POSITION: ${pos}`);
            return [DIRECTION.UNKNOWN, DIRECTION.UNKNOWN];
    }
}

/**
 * @param {NS} ns
 * @param {string[]} data
 */
function find_inside(ns, data) {
    let inside_count = 0;
    let flip = false;
    let connect_south = ['║', '╗', '╔'];
    data.forEach((line) => {
        let pLine = '';

        for (let i = 0; i < line.length; i++) {
            if (connect_south.includes(line[i])) {
                flip = !flip;
                pLine += line[i];
                continue;
            }

            if (line[i] == '.' && flip) {
                inside_count++;
                pLine += line[i];
                continue;
            }

            if (line[i] != '.') {
                pLine += line[i];
                continue;
            }

            pLine += ' ';
        }

        ns.tprintf(pLine);
        flip = false;
    })

    return inside_count;
}