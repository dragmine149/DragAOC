/** @param {NS} ns */
export async function main(ns) {
    // load the data
    // acceptable auguments: ['r', 'd', 'd2', 't'];
    // 'r' = real, 'd' = deafult (part1/part2), 'd2' = default (part2 will do part1 if no part2 found), 't' = test (custom data)
    let data = ns.read(`AOC/Data/2023/12/${ns.args[0]}.txt`);
    let game_data = data.split('\n');
    // remove empty entries from the data. (line has to be completly blank)
    game_data = game_data.filter((v) => v != '');

    let s = performance.now();

    let combinations = 0;
    game_data.forEach((line) => {
        let lComb = process_row(line);
        // ns.tprintf(`${line} : ${lComb}`);
        combinations += lComb;
    })

    let e = performance.now()

    ns.tprintf(`Result: ${combinations}`);
    ns.tprint(`${e - s}ms`);
}

/** @param {String} str */
function to_arr(str) {
    return [...str];
}

/** @param {String} str */
function from_csv(str) {
    // assuming str is a list of numbers
    let s = str.split(',');
    let info = []
    s.forEach((v) => {
        info.push(Number(v));
    })
    return info;
}

/** @param {String} str */
function compress_row(row) {
    // Split row up into a list of unknown, damaged
    let processed = [];
    let prev = '';
    let c = 0;
    to_arr(row).forEach((char) => {
        // don't need to worry about operational ones
        if (char != prev) {
            if (prev != '') {
                // don't want to add nothing
                if (prev == '.') {
                    prev = char;
                    c = 1;
                    return;
                }
                processed.push([prev, c]);
            }
            prev = char;
            c = 0;
        }

        c++;
    })
    if (prev != '.') {
        processed.push([prev, c]);
    }

    return processed;
}


/**
 * @param {String} orow
 */
function process_row(orow) {
    let info = orow.split(' ');
    let row = info[0];
    let numbers = from_csv(info[1]);
    
    // This check happenes with the unfolded
    // In order to do it normally, just replace unfolder stuff
    let unfold = unfold_row(row, numbers);
    let total = 0;
    numbers.forEach((num) => {
        total += num + 1;
    })
    total -= 1; // remove the extra one at the end.
    if (total == row.length) {
        return 1;
    }

    // Passes the original check
    let combo = replace_row(row, numbers, 0);
    console.log(`${row} : ${combo}`);
    // let c2 = replace_row(row + '?', numbers, 0);
    // let c3 = replace_row('?' + row, numbers, 0);

    // Basically for part 2
    // to unfold the spring, we duplicate it 5 times seperated with '?'
    // If it was a char like '.' we can easily complete it
    
    // return c3 * c3 * combo * c2 * c2;
    return combo;
}

/**
 * @param {String} row
 * @param {Number[]} numberData
 */
function check_row(row, numberData) {
    let rData = compress_row(row);
    let success = true;

    // This is what happenes if we don't go though all the cases.
    if (numberData.length > rData.length) {
        return false;
    }

    rData.forEach((item, idx) => {
        if (!success) {
            return;
        }

        // item will always be '#'
        if (numberData[idx] != item[1]) {
            success = false;
            return;
        }
    });

    return success;
}

/**
 * @param {String} row
 * @param {Number[]} rowData
 * @param {Number} startPos
 */
function replace_row(row, rowData, startPos=0) {
    let combos = 0;
    let next = row.indexOf('?', startPos);
    if (next == -1) {
        let r = check_row(row, rowData);
        return r;
    }

    combos += replace_row(row.replace('?', '.'), rowData, next);
    combos += replace_row(row.replace('?', '#'), rowData, next);

    return combos;
}

/**
 * @param {String} row
 * @param {Number[]} numbers
 */
function unfold_row(row, numbers) {
    row += '?';
    row = row.repeat(5);
    row = row.slice(0, -1);
    
    numbers.concat(numbers).concat(numbers).concat(numbers).concat(numbers);

    return {'row': row, 'numbers': numbers};
}