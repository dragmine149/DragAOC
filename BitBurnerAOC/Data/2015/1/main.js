/** @param {NS} ns */
export async function main(ns) {
    // load the data
    // acceptable auguments: ['r', 'd', 'd2', 't'];
    // 'r' = real, 'd' = deafult (part1/part2), 'd2' = default (part2 will do part1 if no part2 found), 't' = test (custom data)
    let data = ns.read(`AOC/Data/2015/1/${ns.args[0]}.txt`);
    let game_data = data.split('\n');
    // remove empty entries from the data. (line has to be completly blank)
    game_data = game_data.filter((v) => v != '');

    ns.tprint(`Instructions take santa to floor: ${translate(game_data)}`);
}

/** @param {String} str */
function to_arr(str) {
    return [...str];
}

/**
 * @param {String[]} data
 */
function translate(data) {
    let position = 0;

    data = to_arr(data[0]);
    for (let charPos = 0; charPos < data.length; charPos++) {
        let char = data[charPos];

        position += char == ')' ? -1 : char == '(' ? 1 : 0;

        if (position == -1) {
            return charPos + 1;
        }
    }

    return position;
}