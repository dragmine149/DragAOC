/** @param {NS} ns */
export async function main(ns) {
    let data = ns.read("AOC/Data/8/r.txt");
    let game_data = data.split('\n');
    game_data = game_data.filter((v) => v != "");

    let instructions = game_data.reverse().pop();
    let program = generate_program(game_data.reverse());

    // ns.tprint(instructions);
    // ns.tprint(program);

    const nodes = [];
    program.forEach((v, k, m) => {
        if (k.endsWith('A')) {
            nodes.push(k);
        }
    })
    const node_moves = new Map();

    ns.tprint(nodes);

    let instruction = 0;

    nodes.forEach((node) => {
        node_moves.set(node, 0);
        let nnode = node;
        instruction = 0;

        while (!nnode.endsWith('Z')) {
            var locInfo = program.get(nnode);
            var instr = instructions[instruction];

            if (instr.toUpperCase() == 'L') {
                nnode = locInfo[0];
            }

            if (instr.toUpperCase() == 'R') {
                nnode = locInfo[1];
            }

            instruction = (instruction + 1) % instructions.length;
            node_moves.set(node, node_moves.get(node) + 1)
        }
    })

    ns.tprint(nodes);
    ns.tprint(node_moves);

    let moves = [];
    node_moves.forEach((move) => {
        moves.push(move);
    })

    ns.tprint(`LCM: ${LCM(moves)}`);
}

/** @param {string[]} data */
function generate_program(data) {
    let result = new Map();

    data.forEach((e) => {
        let info = e.split('=');
        let arr = info[1].trim().replace('(', '').replace(')', '').split(',');
        result.set(info[0].trim(), [arr[0].trim(), arr[1].trim()]);
    })

    return result;
}

function LCM(arr) {
    function gcd(a, b) {
        if (b === 0) return a;
        return gcd(b, a % b);
    }

    let res = arr[0];

    for (let i = 1; i < arr.length; i++) { 
        res = (res * arr[i]) / gcd(res, arr[i]); 
    } 

    return res;
} 