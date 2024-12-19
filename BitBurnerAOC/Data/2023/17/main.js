import { dijkstra, Node, PriorityQueue } from 'AOC/Data/2023/17/algorithm.js';

const DIRECTION = Object.freeze({
    UP: 0,
    LEFT: 1,
    DOWN: 2,
    RIGHT: 3
});

/** @type {Node[][]} */
let boardData = [];

function get_cell(pos) {
    if (pos[0] > 0 && pos[0] <= boardData.length && pos[1] > 0 && pos[1] <= boardData.length) {
        return boardData[pos[1]][pos[0]];
    }

    return -1;
}

/**
 * @param {[Number, Number]} pos
 * @param {Number} size
 */
function convert_pos_index(pos, size) {
    let x = pos[0] + 1;
    let y = pos[1] + 1;
    return (y * size) + x - size - 1;
}

/**
 * @param {[Number, Number]} pos
 * @param {DIRECTION} dir
 * @param {Number} size
 */
function get_surrounding(pos, dir, size) {
    let surrounding = [];
    switch (dir) {
        case DIRECTION.UP:
            surrounding.push([get_cell(pos[0] - 1, pos[1]), convert_pos_index(pos, size), DIRECTION.LEFT]);
            surrounding.push([get_cell(pos[0], pos[1] - 1), convert_pos_index(pos, size), DIRECTION.UP]);
            surrounding.push([get_cell(pos[0] + 1, pos[1]), convert_pos_index(pos, size), DIRECTION.RIGHT]);
            break;
        case DIRECTION.LEFT:
            surrounding.push([get_cell(pos[0], pos[1] - 1), convert_pos_index(pos, size), DIRECTION.DOWN]);
            surrounding.push([get_cell(pos[0] + 1, pos[1]), convert_pos_index(pos, size), DIRECTION.RIGHT]);
            surrounding.push([get_cell(pos[0], pos[1] + 1), convert_pos_index(pos, size), DIRECTION.UP]);
            break;
        case DIRECTION.DOWN:
            surrounding.push([get_cell(pos[0] - 1, pos[1]), convert_pos_index(pos, size), DIRECTION.LEFT]);
            surrounding.push([get_cell(pos[0], pos[1] + 1), convert_pos_index(pos, size), DIRECTION.DOWN]);
            surrounding.push([get_cell(pos[0] + 1, pos[1]), convert_pos_index(pos, size), DIRECTION.RIGHT]);
            break;
        case DIRECTION.RIGHT:
            surrounding.push([get_cell(pos[0], pos[1] + 1), convert_pos_index(pos, size), DIRECTION.DOWN]);
            surrounding.push([get_cell(pos[0] - 1, pos[1]), convert_pos_index(pos, size), DIRECTION.LEFT]);
            surrounding.push([get_cell(pos[0], pos[1] - 1), convert_pos_index(pos, size), DIRECTION.UP]);
            break;
    }
    return surrounding;
}

let list = [];

/** @param {NS} ns */
export async function main(ns) {
    // load the data
    // acceptable auguments: ['r', 'd', 'd2', 't'];
    // 'r' = real, 'd' = deafult (part1/part2), 'd2' = default (part2 will do part1 if no part2 found), 't' = test (custom data)
    let data = ns.read(`AOC/Data/2023/17/${ns.args[0]}.txt`);
    let game_data = data.split('\n');
    // remove empty entries from the data. (line has to be completly blank)
    game_data = game_data.filter((v) => v != '');
    game_data.forEach((line, idx) => {
        let linegrid = [];
        [...line].forEach((char) => {
            linegrid.push(Number(char));
        })
        boardData.push(linegrid);
    })

    let result = dijkstra(get_surrounding, boardData[0].length * boardData.length, [0, 0]);

    ns.tprint("Result: " + String(result.get_dist()));
}