/** @param {NS} ns */
export async function main(ns) {
    let data = ns.read("AOC/Data/9/r.txt");
    let game_data = data.split('\n');
    game_data = game_data.filter((v) => v != "");

    let total = 0;
    game_data.forEach((line) => {
        let seq = generate_sequence(line.split(' '));
        ns.tprint(seq);
        // seq = calculate_next_in_sequence(seq);
        seq = calculate_prev_in_sequence(seq);
        ns.tprint(seq);
        // total += seq[seq.length - 1];
        total += seq[0];
        ns.tprint('----');
    })

    ns.tprint(total);
}

/** @param {String[]} sequence */
function generate_sequence(sequence) {
    let s = [];
    sequence.forEach((i) => {
        s.push(Number(i));
    })

    return s;
}

/** @param {Number[]} sequence */
function calculate_n_sequence(sequence) {
    let sequences = new Map();
    let cSeq = sequence;
    while (!is_sequence_zero(cSeq)) {
        let r = get_sequence_for_numbers(cSeq);
        sequences.set(sequences.size, r);
        cSeq = r;
    }

    /** @type {Number[]} */
    let arr = sequences.get(sequences.size - 1);
    arr.push(0);
    sequences.set(sequences.size - 1, arr);

    return sequences;
}

/** @param {Number[]} sequence */
function calculate_next_in_sequence(sequence) {
    let sequences = calculate_n_sequence(sequence);

    for (let i = sequences.size - 1; i >= 0; i--) {
        /** @type {Number[]} */
        let a = sequences.get(i - 1);
        /** @type {Number[]} */
        let current = sequences.get(i);
        if (a != undefined) {
            a.push(a[a.length - 1] + current[current.length - 1]);
            sequences.set(i - 1, a);
        }
    }

    /** @type {Number[]} */
    let last = sequences.get(0);
    sequence.push(sequence[sequence.length - 1] + last[last.length - 1]);
    return sequence;
}

function calculate_prev_in_sequence(sequence) {
    let sequences = calculate_n_sequence(sequence);

    for (let i = sequences.size - 1; i >= 0; i--) {
        /** @type {Number[]} */
        let nextSeq = sequences.get(i - 1);
        /** @type {Number[]} */
        let current = sequences.get(i);
        if (nextSeq != undefined) {
            let newArr = [nextSeq[0] - current[0]];
            let saveArr = newArr.concat(nextSeq);
            sequences.set(i - 1, saveArr);
        }
    }

    /** @type {Number[]} */
    let last = sequences.get(0);
    let newLast = [sequence[0] - last[0]];
    return newLast.concat(sequence);
}

/** @param {Number[]} sequence */
function is_sequence_zero(sequence) {
    for (let i = 0; i < sequence.length; i++) {
        if (sequence[i] != 0) {
            return false;
        }
    }

    return true;
}

/** @param {Number[]} sequence */
function get_sequence_for_numbers(sequence) {
    let new_sequence = [];
    for (let i = 0; i < sequence.length; i++) {
        let r = get_diff_of_two_numbers(sequence[i], sequence[i + 1]);
        if (!isNaN(r)) {
            new_sequence.push(r);
        }
    }

    return new_sequence;
}

/**
 * @param {Number} a
 * @param {Number} b
 */
function get_diff_of_two_numbers(a, b) {
    return b - a;
}