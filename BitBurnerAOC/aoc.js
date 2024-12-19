import { secondsToDhms, log, terminalInsert, setCSS, addCallback, setNavCommand, getConfiguration, clone, tailWindow } from 'AOC/utils.js';
import { Client } from 'AOC/Client.js';
import { get_aoc_years, get_aoc_year, valid_year, get_aoc_dates, get_aoc_day, valid_day } from 'AOC/calculations.js';

const version = '0.1.0';
/** @type {Document} */
const doc = eval('document');

const Attempt = Object.freeze({
    Correct: 'Correct',
    Low: 'Too Low',
    High: 'Too High',
    Unknown: 'Unknown',
    Recent: 'Too Recent',
    Level: 'Wrong Level'
});

/** @type {{
 * edit: Bool,
 * run: Bool,
 * submit: Bool,
 * attempt: Number,
 * day: Number,
 * year: Number,
 * part: Number
 * overwrite: Bool,
 * I: Bool,
 * P: Bool,
 * v: Bool
 * }} */
const argsSchema = [
    ['edit', false], // File to edit (default: last unlocked day main.js)
    ['run', ''], // File to run (default: last unlocked day main.js)
    ['submit', ''], // Submit the data 
    ['attempt', -1], // Submit an attempt (provide answer) or view the attempts (no-args)
    ['day', 0], // Puzzle day (default: last unlocked day)
    ['year', 0], // Puzzle year (default: year of current or last Advent of Code event)
    ['part', 1], // Which part to do (1 or 2)
    ['overwrite', false], // Overwrite files when downloading if they already exists
    ['I', false], // Download puzzle input only
    ['P', false], // Download puzzle description only
    ['v', false], // Print version information
];

/** @type {argsSchema} */
let options;

/**
 * @param {{
*      servers: string[];
*      txts: string[];
*      scripts: string[];
*      flags: (schema: [string, string | number | boolean | string[]][]) => { [key: string]: string[] | ScriptArg; }
* }} data
* @param {string[]} args
*/
export function autocomplete(data, args) {
    let list = args.slice(args.length - 2);

    /** @type {number[]} */
    const years = get_aoc_years();
    const year = args[args.indexOf('--year') + 1];
    /** @type {number[]} */
    const dates = get_aoc_dates(year);
    const day = args[args.indexOf('--day') + 1];

    if (args.includes('--year') && !years.includes(year)) {
        return get_aoc_years();
    }

    if (args.includes('--day') && !dates.includes(day)) {
        return get_aoc_dates(year);
    }

    /** @type {Number[]} */
    const part = [1, 2];
    const ipart = args[args.indexOf('--part') + 1];

    if (args.includes('--part') && !part.includes(ipart)) {
        return part;
    }

    data.flags(argsSchema);
    return [];
}

function printToTerminal(data) {
    let css = `<style id="aocss">
              .aoc {color:#fff;}
            </style>`
    setCSS('aocss', css);

    data = `<div class="aoc">${data}</div>`;
    terminalInsert(data);
}

let select_code = `<script>
window.addEventListener('click', function(e,s,r){
    if (e.target.nodeName === 'CODE' && e.detail === 3) {
        s = window.getSelection();
        s.removeAllRanges();
        r=document.createRange();
        r.selectNodeContents(e.target);
        s.addRange(r);
    }
});
</script>`

/**
 * @param {NS} ns
 * @param {String} page
 */
function show_page(ns, page) {
    tailWindow(ns, ns.pid, {
        'bc': '#000',
        'c': '#FFFFFF',
        'font': '20px Courier',
        'width': 2000,
        'height': 1000,
        'x': 250,
        'y': 0
    }, page, 'Advent of code');
}

/** @param {NS} ns */
function checks(ns) {
    // Year and day checks
    if (options.year == 0) {
        options.year = get_aoc_year();
    }

    if (options.year < 2015) {
        ns.tprint('ERROR: Advent of code did not exists in the given year!');
        ns.exit();
    }

    if (!valid_year(options.year)) {
        ns.tprint('ERROR: The year entered has not been unlocked yet!');
        ns.exit();
    }

    if (options.day == 0) {
        options.day = get_aoc_day(options.year);
    }

    if (!valid_day(options.year, options.day)) {
        ns.tprint('ERROR: The day entered has not been unlocked yet!');
        ns.exit();
    }

    // loop it back around to the most recent year if negative number
    if (options.year < 0) {
        options.year = get_aoc_year() + options.year;
    }
    if (options.day < 0) {
        options.day = get_aoc_day(options.year) + options.day;
    }

    // Part checks
    if (![1, 2].includes(options.part)) {
        ns.tprint('ERROR: The entered part is not a valid part!');
        ns.exit();
    }
}

/** @type {Client} */
let client;

/** @param {NS} ns */
export async function main(ns) {
    let data = ns.flags(argsSchema);
    ns.tprint(data);
    const runOptions = getConfiguration(ns, argsSchema);
    if (!runOptions) return; // Invalid options, or ran in --help mode.
    options = runOptions;
    checks(ns);

    client = new Client(ns);

    // await create_day(ns, options.year, options.day);

    if (options.edit) {
        setNavCommand(`nano AOC/Data/${options.year}/${options.day}/main.js`);
    }

    if (options.run) {
        setNavCommand(`run AOC/Data/${options.year}/${options.day}/main.js ${options.run}`);
    }

//     if (options.attempt) {
//         if (options.attempt == -1) {
//             // we just print instead.
//             /** @type {{
//          * Part1:[[[Number, Attempt]]], 
//          * Part2:[[[Number, Attempt]]]
//          * }} */
//             let data = ns.read(`AOC/Data/${options.year}/${options.day}/attempt.txt`);
//             ns.tprint(options.day, options.year);
//             ns.tprint(data);
//             console.log(data);
//             data = JSON.parse(data);
//             ns.tprint(data);
//             console.log(data);
//             let p1Text = '';
//             data.Part1.forEach((a) => {
//                 p1Text += `${a[0]}: ${a[1]}\n`;
//             })
//             let p2Text = '';
//             data.Part2.forEach((a) => {
//                 p2Text += `${a[0]}: ${a[1]}\n`;
//             })
//             ns.tprintf(`Attempts for day ${options.day} (${options.year}):
// ---Part 1---
// ${p1Text}
// ---Part 2---
// ${p2Text}`)
//             return;
//         }
//         await client.submitAnswer(options.year, options.day, options.part, options.attempt);
//     }

    if (options.v) {
        ns.tprint(`Advent of code command line interface for bit burner:
Version: ${version}`);
    }

    if (options.submit) {
        if (options.submit != '') {
            ns.tprint(`Submitting answer: ${options.submit} for ${options.year}/day/${options.day}/part/${options.part}`);
            await client.submitAnswer(options.year, options.day, options.part, options.submit);
        }
    }
}

/** 
 * @param {NS} ns  
 * @param {Number} year
 * @param {Number} day
 * */
async function create_day(ns, year, day) {
    const client = new Client(ns);
    const path = `AOC/Data/${year}/${day}/`;
    log(ns, `Creating folder and files. PATH: ${path}`, true);

    // get puzzle input
    let input = await client.getInput(options.year, options.day);
    // get puzzle description
    let puzzle = await client.getDay(options.year, options.day);

    // let input = '';
    // let puzzle = '';

    log(ns, `Creating input`);
    if (!ns.fileExists(path + 'r.txt')) {
        ns.write(path + 'r.txt', input, 'w');
    } else {
        if (options.overwrite) {
            log(ns, `Overwritting input`);
            ns.write(path + 'r.txt', 'data', 'w');
        }
    }

    log(ns, `Creating puzzle`);
    if (!ns.fileExists(path + 'puzzle.txt')) {
        ns.write(path + 'puzzle.txt', puzzle, 'w');
    } else {
        if (options.overwrite) {
            log(ns, `Overwritting puzzle`);
            ns.write(path + 'puzzle.txt', 'data', 'w');
        }
    }

    show_page(ns, puzzle);

    log(ns, `Creating main.js`);
    if (!ns.fileExists(path + 'main.js')) {
        ns.write(path + 'main.js', `/** @param {NS} ns */
export async function main(ns) {
    // load the data
    // acceptable auguments: ['r', 'd', 'd2', 't'];
    // 'r' = real, 'd' = deafult (part1/part2), 'd2' = default (part2 will do part1 if no part2 found), 't' = test (custom data)
    let data = ns.read("AOC/Data/${year}/${day}/#.txt");
    let game_data = data.split('~n');
    // remove empty entries from the data. (line has to be completly blank)
    game_data = game_data.filter((v) => v != '');
}
`.replace('~', '\\').replace('#', '${ns.args[0]}').replaceAll('"', '`'));
    }

    log(ns, `Creating data files`);
    if (!ns.fileExists(path + 'attempt.txt')) {
        /** @type {{
         * Part1:[[Number, Attempt][]], 
         * Part2:[[Number, Attempt][]]
         * }} */
        let defattempt = { 'Part1': [], 'Part2': [] };
        ns.write(path + 'attempt.txt', JSON.stringify(defattempt), 'w');
    }
    // Over files will be created if called to be the command.
}