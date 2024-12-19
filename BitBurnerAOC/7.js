/** @param {NS} ns */
export async function main(ns) {
    let data = ns.read("AOC/Data/7r.txt");
    let game_data = data.split('\n');
    game_data = game_data.filter((v) => v != "");

    // game_data.forEach((value) => {
    //     let hand = value.split(' ')[0];
    //     ns.tprint({ hand: hand, type: get_hand_type(hand) });
    // })

    game_data.sort(function (a, b) {
        let aHand = a.split(' ')[0];
        let bHand = b.split(' ')[0];

        // let aType = get_hand_type(aHand);
        // let bType = get_hand_type(bHand);
        let aType = convert_jockers(aHand);
        let bType = convert_jockers(bHand);

        if (aType == "ERROR") {
            ns.tprint(`ERROR: ${aHand}`);
            ns.exit();
        }

        if (bType == "ERROR") {
            ns.tprint(`ERROR: ${bHand}`);
            ns.exit();
        }

        if (aType > bType) {
            return 1;
        }

        if (aType < bType) {
            return -1;
        }

        for (let i = 0; i < 5; i++) {
            let result = compare_cards(aHand[i], bHand[i]);
            if (result == 1) {
                return -1;
            }

            if (result == -1) {
                return 1;
            }
        }
        // assuming no two hands are the same
    });

    ns.tprint("Sorted data:");
    ns.tprint(game_data);

    // Get bid value
    let value = 0;
    for (let i = 0; i < game_data.length; i++) {
        value += (Number(game_data[i].split(' ')[1]) * (i + 1));
    }

    ns.tprint("Game Value:");
    ns.tprint(value);
}


// const strength = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"];
const strength = ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J"];

/**
 * @param {String} cardA
 * @param {String} cardB
 */
function compare_cards(cardA, cardB) {
    let cardAstr = strength.findIndex((v) => v == cardA);
    let cardBstr = strength.findIndex((v) => v == cardB);

    if (cardAstr > cardBstr) {
        return 1;
    }

    if (cardAstr < cardBstr) {
        return -1;
    }

    return 0;
}

/** @param {String} hand */
function convert_jockers(hand) {
    let aHand = [...hand];
    let mScore = 0;
    aHand.forEach((card) => {
        if (card == 'J') {
            return;
        }

        let rhand = hand.replaceAll('J', card);
        let hScore = get_hand_type(rhand);
        if (hScore > mScore) {
            mScore = hScore;
        }
    })

    if (mScore == 0) {
        mScore = 7;
    }

    return mScore;
}

/**
 * @param {String} hand
 */
function compress_hand(hand) {
    let compressed = [];
    let last_letter = hand[0];
    let letters = new Map();
    let count = 0;
    for (let letterIndex in hand) {
        let letter = hand[letterIndex];

        if (letters.get(letter) == undefined) {
            letters.set(letter, 0);
        }

        letters.set(letter, letters.get(letter) + 1);

        if (letter == last_letter) {
            count++;
            continue;
        }

        compressed.push([last_letter, count]);
        last_letter = letter;
        count = 1;
    }

    compressed.push([last_letter, count]);
    return { "c": compressed, "l": letters };
}

/**
 * @param {String} hand
 */
function get_hand_type(hand) {
    let r = compress_hand(hand);
    let compress = r.c;
    let letters = r.l;
    // console.log(compress, letters);
    // console.log('--');

    if (compress[0][1] == 5) {
        return 7;
    }

    if (letters.size == 2) {
        for (let value of letters.values()) {
            if (value == 4) {
                return 6;
            }
            if (value == 3) {
                return 5;
            }
        }
    }

    if (letters.size == 3) {
        let two_pair = false;
        for (let value of letters.values()) {
            if (value == 3) {
                return 4;
            }
            if (value == 2) {
                if (!two_pair) {
                    two_pair = true;
                    continue;
                }

                return 3;
            }
        }

        if (two_pair) {
            return 2;
        }
    }

    let pair2 = false;
    for (let v in compress.values()) {
        if (v[1] == 2) {
            if (!pair2) {
                pair2 = true;
                continue;
            }
            return 3;
        }
    }

    if (letters.size == 4) {
        return 2;
    }

    if (letters.size == 5) {
        return 1;
    }

    return "ERROR";
}