/** @param {NS} ns */
export async function main(ns) {
    // load the data
    // acceptable auguments: ['r', 'd', 'd2', 't'];
    // 'r' = real, 'd' = deafult (part1/part2), 'd2' = default (part2 will do part1 if no part2 found), 't' = test (custom data)
    let data = ns.read(`AOC/Data/2023/15/${ns.args[0]}.txt`);
    let game_data = data.split('\n');
    // remove empty entries from the data. (line has to be completly blank)
    game_data = game_data.filter((v) => v != '');
    game_data = game_data[0].split(',');

    // PART 1
    // let sum = 0;
    // game_data.forEach((step) => {
    //     let result = hash_algorithm(step);
    //     ns.tprintf(`Step: ${step}. Result: ${result}`);
    //     sum += result;
    // })

    // PART 2
    /** @type {[[[String, Number]]]} */
    // let boxes = new Array(256).fill([]);
    let boxes = [];
    for (let i = 0; i < 256; i++) {
        boxes.push([]);
    }


    // const REG = /([a-zA-Z]+)([-=])(?:(?<==)(\d+))?/g;
    console.log(game_data);
    game_data.forEach((step) => {
        // console.log(step);
        // const data = REG.exec(step.trim());
        // if (data == null) {
        //     ns.tprint(`ERROR: INVALID DATA: ${step}`);
        //     ns.exit();
        // }

        // const label = data[1];
        // const sym = data[2];
        // const num = Number(data[3]);
        // ns.tprint([label, sym, num]);
        // let label_hash = hash_algorithm(label);

        let splitPos = step.indexOf('-');
        let sym = '-';
        if (splitPos < 0) {
            splitPos = step.indexOf('=');
            sym = '=';
        }
        if (splitPos < 0) {
            ns.tprintf(`ERROR: PANIC! NO VALID SPLIT POSITION FOUND IN ${step}`);
            ns.exit();
        }

        let labelSplit = step.split(sym);
        let label = labelSplit[0];
        let num = labelSplit[1];
        // ns.tprintf(`Label: ${label}`);
        let box = hash_algorithm(labelSplit[0]);
        let index;

        switch (sym) {
            case '-':
                index = boxes[box].findIndex((v) => v[0] == label);
                if (index >= 0) {
                    boxes[box].splice(index, 1);
                    break;
                }
                break;
            case '=':
                let focal = Number(num);
                index = boxes[box].findIndex((v) => v[0] == label);
                if (index >= 0) {
                    boxes[box][index][1] = focal;
                    break;
                }

                boxes[box].push([label, focal]);
                break;
            default:
                ns.tprint(`ERROR: ${sym} is not a valid symbol!`);
                ns.exit();
                break;
        }
        // ns.tprint(boxes);
    })

    let power = 0;
    boxes.forEach((box, bdx) => {
        box.forEach((lens, ldx) => {
            power += ((bdx + 1) * (ldx + 1) * lens[1]);
        })
    })

    // ns.tprint(`Overall result: ${sum}`);
    ns.tprint(`Overall result: ${power}`);
}

/**
 * @param {String[]} str
 * @param {Number} value
 */
function hash_algorithm(str, value = 0) {
    if (typeof str == 'string') {
        str = [...str];
    }

    let char = str.splice(0, 1)[0];
    value += char.charCodeAt();
    value *= 17;
    value %= 256;

    if (str.length > 0) {
        value = hash_algorithm(str, value);
    }

    return value;
}