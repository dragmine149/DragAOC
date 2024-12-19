// attach the .equals method to Array's prototype to call it on any array
/**
 * @param {any[]} a
 * @param {any[]} b
 */
function equals(a, b) {
    // if the other array is a falsy value, return
    if (!a)
        return false;
    // if the argument is the same array, we can be sure the contents are same as well
    if(a === b)
        return true;
    // compare lengths - can save a lot of time 
    if (b.length != a.length)
        return false;

    for (var i = 0, l=b.length; i < l; i++) {
        // Check if we have nested arrays
        if (b[i] instanceof Array && a[i] instanceof Array) {
            // recurse into the nested arrays
            if (!b[i].equals(a[i]))
                return false;       
        }           
        else if (b[i] != a[i]) { 
            // Warning - two different object instances will never be equal: {x:20} != {x:20}
            return false;   
        }           
    }       
    return true;
}

/**
 * @param {any[][]} arr
 * @param {any[]} chk_arr
 */
function includes_arr(arr, chk_arr) {
    let includes = false;
    arr.forEach((i) => {
        if (includes) {
            return;
        }
        includes = equals(i, chk_arr);
    })

    return includes;
}

// let numbers = new Map();
let numbers = [];

/** @param {NS} ns */
export async function main(ns) {
    // load the data
    // acceptable auguments: ['r', 'd', 'd2', 't'];
    // 'r' = real, 'd' = deafult (part1/part2), 'd2' = default (part2 will do part1 if no part2 found), 't' = test (custom data)
    let data = ns.read(`AOC/Data/2023/3/${ns.args[0]}.txt`);
    let game_data = data.split('\n');
    // remove empty entries from the data. (line has to be completly blank)
    game_data = game_data.filter((v) => v != "");
    
    
    numbers = [];
    process(game_data);
    
    let sum = 0;
    let checked_gears = [];
    numbers.forEach((num) => {
        // ns.tprintf(num[0]);
        // sum += Number(num[0]);

        if (includes_arr(checked_gears, num[1])) {
            return;
        }

        let gears = numbers.filter((v) => equals(v[1], num[1]));
        ns.tprint(gears);
        if (gears.length != 2) {
            return;
        }

        checked_gears.push(gears[0][1]);

        sum += (Number(gears[0][0]) * Number(gears[1][0]));

    })

    ns.tprint(`Result: ${sum}`);
}


/**
 * @param {string[]} data
 * @param {number[]} pos (x = 0, y = 1)
 */
function get_surrounding_cells(data, pos) {
    var surrounding = [];
    let x = pos[0];
    let y = pos[1];

    if (y - 1 >= 0) {
        if (x - 1 >= 0) {
            surrounding.push(data[y - 1][x - 1]);
        }
        surrounding.push(data[y - 1][x]);
        if (x + 1 < data[0].length) {
            surrounding.push(data[y - 1][x + 1]);
        }
    }

    if (x - 1 >= 0) {
        surrounding.push(data[y][x - 1]);
    }
    if (x + 1 < data[0].length) {
        surrounding.push(data[y][x + 1]);
    }

    if (y + 1 < data.length) {
        if (x - 1 >= 0) {
            surrounding.push(data[y + 1][x - 1]);
        }
        surrounding.push(data[y + 1][x]);
        if (x + 1 < data[0].length) {
            surrounding.push(data[y + 1][x + 1]);
        }
    }

    return surrounding;
}

/**
 * @param {string[]} cells
 */
function get_symbol(cells) {
    return cells.filter((sym) => ['#', '-', '+', '@', '/', '%', '&', '*', '$', '='].includes(sym));
}

/**
 * @param {String[]} cells
 * @param {String} sym
 * @param {Number} x
 * @param {Number} y
 */
function get_symbol_pos(data, sym, x, y) {
    if (y - 1 >= 0) {
        if (x - 1 >= 0) {
            if (data[y - 1][x - 1] == sym)
                return [x - 1, y - 1];
        }
        if (x + 1 < data[0].length) {
            if (data[y - 1][x + 1] == sym) {
                return [x + 1, y - 1];
            }
        }

        if (data[y - 1][x] == sym) {
            return [x, y - 1];
        }
    }

    if (x - 1 >= 0) {
        if (data[y][x - 1] == sym) {
            return [x - 1, y];
        }
    }
    if (x + 1 < data[0].length) {
        if (data[y][x + 1] == sym) {
            return [x + 1, y];
        }
    }

    if (y + 1 < data.length) {
        if (x - 1 >= 0) {
            if (data[y + 1][x - 1] == sym) {
                return [x - 1, y + 1];
            }
        }
        if (x + 1 < data[0].length) {
            if (data[y + 1][x + 1] == sym) {
                return [x + 1, y + 1];
            }
        }

        if (data[y + 1][x] == sym) {
            return [x, y + 1];
        }
    }
    return [0, 0];

    switch (cells.indexOf(sym)) {
        case 0:
            return [x - 1, y - 1];
        case 1:
            return [x, y - 1];
        case 2:
            return [x + 1, y - 1];
        case 3:
            return [x - 1, y];
        case 4:
            return [x + 1, y];
        case 5:
            return [x - 1, y + 1];
        case 6:
            return [x, y + 1];
        case 7:
            return [x + 1, y + 1];
        default:
            // ??
            return [x, y];
    }
}

/**
 * @param {String} str
 */
function isNumeric(str) {
    if (typeof str != "string") return false // we only process strings!  
    return !isNaN(str) && // use type coercion to parse the _entirety_ of the string (`parseFloat` alone does not do this)...
        !isNaN(parseFloat(str)) // ...and ensure strings of whitespace fail
}

/** @param {String} str */
function to_arr(str) {
    return [...str];
}

/**
 * @param {string[]} data
 */
function process(data) {
    to_arr(data).forEach((line, line_pos) => {
        let temp_num = '';
        let add_to_sum = false;
        let sym_pos = [-1, -1];
        to_arr(line).forEach((element, element_pos) => {
            // process for a valid int
            if (isNumeric(element)) {
                temp_num += element;
                let cells = get_surrounding_cells(data, [element_pos, line_pos]);
                let sym = get_symbol(cells);
                // console.log(`${data[line_pos][element_pos]} [${element_pos}, ${line_pos}]: ${sym} (cells: ${cells})`);
                // if (sym.length > 0) {
                if (sym.includes('*')) {
                    add_to_sum = true;
                    // sym_pos = get_symbol_pos(cells, sym[0], element_pos, line_pos);
                    sym_pos = get_symbol_pos(data, '*', element_pos, line_pos);
                    return;
                }

                return;
            }

            // Otherwise, reset and add to sum
            if (temp_num != '') {
                if (add_to_sum) {
                    add_to_sum = false;
                    numbers.push([temp_num, sym_pos]);
                    // console.log('--')
                }

                temp_num = '';
            }
        })

        if (temp_num != '') {
            if (add_to_sum) {
                numbers.push([temp_num, sym_pos]);
            }
        }
    })
}