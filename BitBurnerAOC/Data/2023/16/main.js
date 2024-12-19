const DIRECTION = Object.freeze({
    UP: 0,
    LEFT: 1,
    DOWN: 2,
    RIGHT: 3
})

const TileType = Object.freeze({
    EMPTY: '.',
    MIRRORLU: '/',
    MIRRORDR: '╚',
    SPLITLR: '-',
    SPLITUD: '|'
});

class Tile {
    #UP = false;
    #LEFT = false;
    #DOWN = false;
    #RIGHT = false;
    #TYPE;

    /** @param {String} type */
    constructor(type) {
        this.#UP = false;
        this.#LEFT = false;
        this.#DOWN = false;
        this.#RIGHT = false;

        switch (type) {
            case '.':
                this.#TYPE = TileType.EMPTY;
                break;
            case '/':
                this.#TYPE = TileType.MIRRORLU;
                break;
            case '\\':
                this.#TYPE = TileType.MIRRORDR;
                break;
            case '-':
                this.#TYPE = TileType.SPLITLR;
                break;
            case '|':
                this.#TYPE = TileType.SPLITUD;
        }
    }

    get_type() {
        return this.#TYPE;
    }

    has_visited() {
        return this.#UP || this.#LEFT || this.#DOWN || this.#RIGHT;
    }

    visit_count() {
        let count = 0;
        count += this.#UP ? 1 : 0;
        count += this.#LEFT ? 1 : 0;
        count += this.#DOWN ? 1 : 0;
        count += this.#RIGHT ? 1 : 0;
        return count;
    }


    /** 
     * @param {DIRECTION} direction 
     * @return {Bool}
     */
    from_dir(direction) {
        switch (direction) {
            case DIRECTION.UP:
                return this.#UP;
            case DIRECTION.LEFT:
                return this.#LEFT;
            case DIRECTION.DOWN:
                return this.#DOWN;
            case DIRECTION.RIGHT:
                return this.#RIGHT;
        }
    }

    /** 
     * @param {DIRECTION} direction 
     */
    set_dir(direction) {
        switch (direction) {
            case DIRECTION.UP:
                this.#UP = true;
                break;
            case DIRECTION.LEFT:
                this.#LEFT = true;
                break;
            case DIRECTION.DOWN:
                this.#DOWN = true;
                break;
            case DIRECTION.RIGHT:
                this.#RIGHT = true;
                break;
        }
    }

    reset() {
        this.#UP = false;
        this.#LEFT = false;
        this.#DOWN = false;
        this.#RIGHT = false;
    }
}

let list = [];
/** @type {Tile[][]} */
let boardData = [];

/** @param {NS} ns */
function printBoard(ns) {
    boardData.forEach((line) => {
        let pline = '';
        line.forEach((tile) => {
            // pline += tile.has_visited() ? '#' : '.';
            pline += tile.visit_count();
        })
        ns.tprintf(pline);
    })
    ns.tprintf('*'.repeat(boardData[0].length));
}

/** @param {NS} ns */
function printBoardR(ns) {
    boardData.forEach((line) => {
        ns.tprintf(line);
    })
    ns.tprintf('*'.repeat(boardData[0].length));
}

/**
 * @param {NS} ns
 * @param {String[]} original
 */
function printSide(ns, original) {
    boardData.forEach((line, idx) => {
        let pline = original[idx] + '\t';
        line.forEach((tile) => {
            pline += tile.visit_count();
        })
        ns.tprintf(pline);
    })
    ns.tprintf('*'.repeat(boardData[0].length) + '\t' + '*'.repeat(boardData[0].length));
}

/**
 * @param {Number} idx
 * @param {DIRECTION} dir
 * @param {Number} size
 */
function index_from_loop_dir(idx, dir, size) {
    switch (dir) {
        case DIRECTION.UP:
            return [idx, size - 1];
        case DIRECTION.LEFT:
            return [size - 1, idx];
        case DIRECTION.DOWN:
            return [idx, 0];
        case DIRECTION.RIGHT:
            return [0, idx];
    }
}

/** @param {NS} ns */
export async function main(ns) {
    // load the data
    // acceptable auguments: ['r', 'd', 'd2', 't'];
    // 'r' = real, 'd' = deafult (part1/part2), 'd2' = default (part2 will do part1 if no part2 found), 't' = test (custom data)
    let data = ns.read(`AOC/Data/2023/16/${ns.args[0]}.txt`);
    let game_data = data.split('\n');
    // remove empty entries from the data. (line has to be completly blank)
    game_data = game_data.filter((v) => v != '');
    boardData = game_data;
    // printBoardR(ns);
    list = [];

    let scanned = [];
    for (let i = 0; i < game_data.length; i++) {
        let temp = [];
        for (let j = 0; j < game_data[i].length; j++) {
            temp.push(new Tile(game_data[i][j]));
        }
        scanned.push(temp);
    }
    boardData = scanned;

    let score = [];
    let start = performance.now();

    Object.values(DIRECTION).forEach((dir) => {
        for (let i = 0; i < game_data.length; i++) {
            // reset 
            boardData.forEach((line) => {
                line.forEach((tile) => {
                    tile.reset();
                })
            })
            list = [];

            // main stuff now
            // list.push([[0, 0], DIRECTION.RIGHT]);
            list.push([index_from_loop_dir(i, dir, game_data.length), dir]);
            ns.tprint(list[0]);
            mirror(ns);

            let energised = 0;
            boardData.forEach((line) => {
                line.forEach((tile) => {
                    energised += tile.has_visited() ? 1 : 0;
                })
            })
            // and the end
            score.push(energised);
        }
    })

    let end = performance.now();

    ns.tprint(`Scores: ${score}`);
    ns.tprint(`Result: ${Math.max(...score)}`);
    ns.tprint(`Time: ${end - start}ms`);
    // printBoard(ns);
    // printSide(ns, game_data);
}


/**
 * @param {[Number, Number]} pos
 * @param {DIRECTION} direction
 */
function convert_pos_direction(pos, direction) {
    switch (direction) {
        case DIRECTION.UP:
            return [pos[0], pos[1] - 1];
        case DIRECTION.LEFT:
            return [pos[0] - 1, pos[1]];
        case DIRECTION.DOWN:
            return [pos[0], pos[1] + 1];
        case DIRECTION.RIGHT:
            return [pos[0] + 1, pos[1]];
    }
}

/** @param {[Number, Number]} pos */
function in_range(pos) {
    return !(pos[1] < 0 || pos[1] >= boardData.length || pos[0] >= boardData[0].length || pos[0] < 0);
}

/** @param {NS} ns */
function mirror(ns) {
    // console.log(list);
    while (list.length > 0) {
        // printBoard(ns);
        // ns.tprint(list);
        let data = list.splice(0, 1)[0];
        let pos = data[0];
        let direction = data[1];

        if (!in_range(pos)) {
            continue; // end of path don't go down here
        }

        boardData[pos[1]][pos[0]].set_dir(direction);

        switch (boardData[pos[1]][pos[0]].get_type()) {
            case TileType.EMPTY:
                let newPos = convert_pos_direction(pos, direction)
                if (in_range(newPos)) {
                    if (!boardData[newPos[1]][newPos[0]].from_dir(direction)) {
                        list.push([newPos, direction]);
                    }
                }
                break;
            case TileType.SPLITUD:
                if (direction == DIRECTION.LEFT || direction == DIRECTION.RIGHT) {
                    let upPos = convert_pos_direction(pos, DIRECTION.UP);
                    if (in_range(upPos)) {
                        if (!boardData[upPos[1]][upPos[0]].from_dir(DIRECTION.UP)) {
                            list.push([upPos, DIRECTION.UP]);
                        }
                    }

                    let downPos = convert_pos_direction(pos, DIRECTION.DOWN);
                    if (in_range(downPos)) {
                        if (!boardData[downPos[1]][downPos[0]].from_dir(DIRECTION.DOWN)) {
                            list.push([downPos, DIRECTION.DOWN]);
                        }
                    }
                    break;
                }
                // this is to carry on like normal
                let verPOS = convert_pos_direction(pos, direction)
                if (in_range(verPOS)) {
                    if (!boardData[verPOS[1]][verPOS[0]].from_dir(direction)) {
                        list.push([verPOS, direction]);
                    }
                }
                break;
            case TileType.SPLITLR:
                if (direction == DIRECTION.UP || direction == DIRECTION.DOWN) {
                    let lp = convert_pos_direction(pos, DIRECTION.LEFT);
                    if (in_range(lp)) {
                        if (!boardData[lp[1]][lp[0]].from_dir(DIRECTION.LEFT)) {
                            list.push([convert_pos_direction(pos, DIRECTION.LEFT), DIRECTION.LEFT]);
                        }
                    }
                    let rp = convert_pos_direction(pos, DIRECTION.RIGHT);
                    if (in_range(rp)) {
                        if (!boardData[rp[1]][rp[0]].from_dir(DIRECTION.RIGHT)) {
                            list.push([convert_pos_direction(pos, DIRECTION.RIGHT), DIRECTION.RIGHT]);
                        }
                    }
                    break;
                }
                // this is to carry on like normal
                let horPos = convert_pos_direction(pos, direction);
                if (in_range(horPos)) {
                    if (!boardData[horPos[1]][horPos[0]].from_dir(direction)) {
                        list.push([horPos, direction]);
                    }
                }
                break;
            // \
            case TileType.MIRRORDR:
                switch (direction) {
                    case DIRECTION.RIGHT:
                        let ddp = convert_pos_direction(pos, DIRECTION.DOWN);
                        if (in_range(ddp)) {
                            if (!boardData[ddp[1]][ddp[0]].from_dir(DIRECTION.DOWN)) {
                                list.push([ddp, DIRECTION.DOWN]);
                            }
                        }
                        break;
                    case DIRECTION.DOWN:
                        let rdp = convert_pos_direction(pos, DIRECTION.RIGHT);
                        if (in_range(rdp)) {
                            if (!boardData[rdp[1]][rdp[0]].from_dir(DIRECTION.RIGHT)) {
                                list.push([rdp, DIRECTION.RIGHT]);
                            }
                        }
                        break;
                    case DIRECTION.LEFT:
                        let udp = convert_pos_direction(pos, DIRECTION.UP);
                        if (in_range(udp)) {
                            if (!boardData[udp[1]][udp[0]].from_dir(DIRECTION.UP)) {
                                list.push([udp, DIRECTION.UP]);
                            }
                        }
                        break;
                    case DIRECTION.UP:
                        let ldp = convert_pos_direction(pos, DIRECTION.LEFT);
                        if (in_range(ldp)) {
                            if (!boardData[ldp[1]][ldp[0]].from_dir(DIRECTION.LEFT)) {
                                list.push([ldp, DIRECTION.LEFT]);
                            }
                        }
                        break;
                }
                break;
            // /
            case TileType.MIRRORLU:
                switch (direction) {
                    case DIRECTION.RIGHT:
                        let udp = convert_pos_direction(pos, DIRECTION.UP);
                        if (in_range(udp)) {
                            if (!boardData[udp[1]][udp[0]].from_dir(DIRECTION.UP)) {
                                list.push([udp, DIRECTION.UP]);
                            }
                        }
                        break;
                    case DIRECTION.DOWN:
                        let ldp = convert_pos_direction(pos, DIRECTION.LEFT);
                        if (in_range(ldp)) {
                            if (!boardData[ldp[1]][ldp[0]].from_dir(DIRECTION.LEFT)) {
                                list.push([ldp, DIRECTION.LEFT]);
                            }
                        }
                        break;
                    case DIRECTION.LEFT:
                        let ddp = convert_pos_direction(pos, DIRECTION.DOWN);
                        if (in_range(ddp)) {
                            if (!boardData[ddp[1]][ddp[0]].from_dir(DIRECTION.DOWN)) {
                                list.push([ddp, DIRECTION.DOWN]);
                            }
                        }
                        break;
                    case DIRECTION.UP:
                        let rdp = convert_pos_direction(pos, DIRECTION.RIGHT);
                        if (in_range(rdp)) {
                            if (!boardData[rdp[1]][rdp[0]].from_dir(DIRECTION.RIGHT)) {
                                list.push([rdp, DIRECTION.RIGHT]);
                            }
                        }
                        break;
                }
                break;
        }
    }
}