function today() {
    let tdy = new Date();
    tdy.setHours(tdy.getUTCHours() - 5); // offset the date to match the AOC timezone
    return tdy;
}

export function get_aoc_years() {
    let diff = today() - new Date(2015, 1);
    let year = new Date(diff).getFullYear() - 1970;
    let arr = [];
    for (let i = 0; i < year; i++) {
        arr.push(2015 + i);
    }

    if (today().getMonth() == 11) {
        arr.push(today().getFullYear());
    }

    return arr;
}

export function get_aoc_year() {
    if (today().getMonth() == 11) {
        return today().getFullYear();
    }

    return today().getFullYear() - 1;
}

export function valid_year(year) {
    let last_valid = get_aoc_year();
    return year <= last_valid;
}

/** @param {Number} */
export function get_aoc_dates(year) {
    if (year == undefined) {
        return []; // can't do calculations without a year
    }

    if (today().getFullYear() != year) {
        return [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];
    }

    if (today().getMonth() != 11) {
        return []; // can't get the date of month if it's not out yet
    }

    let days = today().getDate();
    let arr = [];
    for (let i = 0; i < days; i++) {
        arr.push(i + 1);
    }

    return arr;
}

export function get_aoc_day(year) {
    if (today().getFullYear() != year) {
        return 25;
    }

    return today().getDate();
}

export function valid_day(year, day) {
    let last_valid = get_aoc_day(year);
    return day <= last_valid;
}