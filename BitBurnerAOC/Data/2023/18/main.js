/** @param {NS} ns */
export async function main(ns) {
    // load the data
    // acceptable auguments: ['r', 'd', 'd2', 't'];
    // 'r' = real, 'd' = deafult (part1/part2), 'd2' = default (part2 will do part1 if no part2 found), 't' = test (custom data)
    let data = ns.read(`AOC/Data/2023/18/${ns.args[0]}.txt`);
    let game_data = data.split('\n');
    // remove empty entries from the data. (line has to be completly blank)
    game_data = game_data.filter((v) => v != '');
    
    let boardData = create_board(game_data);
    boardData.forEach((corner) => {
        ns.tprint(corner);
    })
}

/**
 * @param {String[]} data
 */
function create_board(data) {
    let coords = [];
    let current = [0, 0]; // x,y
    data.forEach((instruction) => {
        let info = instruction.split(' ');
        let dir = info[0];
        let count = info[1];
        let colour = info[2];

        let newCoords = current;
        switch (dir) {
            case 'R':
                newCoords = [current[0] + Number(count), current[1]];
                break;
            case 'D':
                newCoords = [current[0], current[1] + Number(count)];
                break;
            case 'L':
                newCoords = [current[0] - Number(count), current[1]];
                break;
            case 'U':
                newCoords = [current[0], current[1] - Number(count)];
                break;
            default:
                console.log(`Unknown dir: ${dir}`);
                break;
        }

        current = newCoords;
        coords.push([current, colour.replace('(', '').replace(')', '')]);
    })

    return coords;
}

/**
 * @param {[[Number, Number], String][]} boardData
 */
function calculate_area(boardData) {
    
}