/** @param {NS} ns */
export async function main(ns) {
    let data = ns.read("AOC/Data/8/r.txt");
    let game_data = data.split('\n');
    game_data = game_data.filter((v) => v != "");

    let instructions = game_data.reverse().pop();
    let program = generate_program(game_data.reverse());

    ns.tprint(instructions);
    ns.tprint(program);

    const start = 'AAA';
    const end = 'ZZZ';
    let location = 'AAA';
    let instruction = 0;
    let moves = 0;

    while (location != end) {
        var locInfo = program[location];
        var instr = instructions[instruction];

        console.log(locInfo);
        console.log(instr);

        if (instr.toUpperCase() == 'L') {
            location = locInfo[0];
        }

        if (instr.toUpperCase() == 'R') {
            location = locInfo[1];
        }

        instruction = (instruction + 1) % instructions.length;
        moves++;
    }

    ns.tprint(moves);
}

/** @param {string[]} data */
function generate_program(data) {
    let result = {

    }

    data.forEach((e) => {
        let info = e.split('=');
        let arr = info[1].trim().replace('(', '').replace(')', '').split(',');
        result[info[0].trim()] = [arr[0].trim(), arr[1].trim()];
    })

    return result;
}