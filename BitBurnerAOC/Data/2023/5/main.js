/** @param {NS} ns */
export async function main(ns) {
    // load the data
    // acceptable auguments: ['r', 'd', 'd2', 't'];
    // 'r' = real, 'd' = deafult (part1/part2), 'd2' = default (part2 will do part1 if no part2 found), 't' = test (custom data)
    let data = ns.read(`AOC/Data/2023/5/${ns.args[0]}.txt`);
    let game_data = data.split('\n');
    // remove empty entries from the data. (line has to be completly blank)
    game_data = game_data.filter((v) => v != '');
    create_maps_from_data(game_data);
    process();
}

/** @type {Number[][]} */
let seeds;
/** @type {Number[][]} */
let seeds_to_soil;
/** @type {Number[][]} */
let soil_to_fertilizer;
/** @type {Number[][]} */
let fertilizer_to_water;
/** @type {Number[][]} */
let water_to_light;
/** @type {Number[][]} */
let light_to_temperature;
/** @type {Number[][]} */
let temperature_to_humidity;
/** @type {Number[][]} */
let humidity_to_location;

/**
 * @param {String[]} game_data
 */
function create_maps_from_data(game_data) {
    let tempArr = [];
    seeds = [];
    let tempSeeds = game_data[0].replace('seeds:', '').split(' ');
    for (let sdx = 0; sdx < tempSeeds.length; sdx += 2) {
        if (tempSeeds[sdx].trim() == '') {
            sdx -= 1;
            continue;
        }
        seeds.push([Number(tempSeeds[sdx]), Number(tempSeeds[sdx + 1])]);
    }

    game_data.splice(0, 1);

    game_data.forEach((line) => {
        switch (line) {
            case 'seed-to-soil map:':
                tempArr = [];
                break;
            case 'soil-to-fertilizer map:':
                seeds_to_soil = tempArr;
                seeds_to_soil.sort((a, b) => a[1] - b[1]);
                tempArr = [];
                break;
            case 'fertilizer-to-water map:':
                soil_to_fertilizer = tempArr;
                soil_to_fertilizer.sort((a, b) => a[1] - b[1]);
                tempArr = [];
                break;
            case 'water-to-light map:':
                fertilizer_to_water = tempArr;
                fertilizer_to_water.sort((a, b) => a[1] - b[1]);
                tempArr = [];
                break;
            case 'light-to-temperature map:':
                water_to_light = tempArr;
                water_to_light.sort((a, b) => a[1] - b[1]);
                tempArr = [];
                break;
            case 'temperature-to-humidity map:':
                light_to_temperature = tempArr;
                light_to_temperature.sort((a, b) => a[1] - b[1]);
                tempArr = [];
                break;
            case 'humidity-to-location map:':
                temperature_to_humidity = tempArr;
                temperature_to_humidity.sort((a, b) => a[1] - b[1]);
                tempArr = [];
                break;
            case '':
                break;
            default:
                let lineInfo = line.split(' ');
                tempArr.push([Number(lineInfo[0]), Number(lineInfo[1]), Number(lineInfo[2])]);
                break;
        }
    })
    humidity_to_location = new Array(tempArr);
    humidity_to_location.sort((a, b) => a[0] - b[0]);
}


function process() {
    seeds.forEach((group) => {
        console.log(group);
        let soil = map_range(group, seeds_to_soil);
        console.log(soil);
        let fertilizer = map_range(soil[1], soil_to_fertilizer);
        console.log(fertilizer);
        let water = map_range(fertilizer, fertilizer_to_water);
        console.log(water);
        let light = map_range(water, water_to_light);
        console.log(light);
        let temperature = map_range(light, light_to_temperature);
        console.log(temperature);
        let humidity = map_range(temperature, temperature_to_humidity);
        console.log(humidity);
        let location = map_range(humidity, humidity_to_location);
        console.log(location);
    })
}


/**
 * @param {[Number, Number]} range
 * @param {Number[]} entries
 * @return {[[Number, Number], [Number, Number], [Number, Number]]}
 */
function map_range(range, entries) {
    let result = [];
    for (let entry = 0; entry < entries.length; entry++) {
        if (range.length == 0) {
            break;
        }
        let [before, mapped, after] = process_entry(range, entries[entry]);

        if (before.length != 0) {
            result.push(before);
        } else {
            result.push([]);
        }

        if (mapped.length != 0) {
            result.push(mapped);
        } else {
            result.push([]);
        }

        range = after;
    }

    if (range.length != 0) {
        result.push(range);
    } else {
        result.push([]);
    }

    return result;
}

/**
 * @param {[Number, Number]} range (Start, Length)
 * @param {[Number, Number, Number]} entry (Destination, Source, Range)
 */
function process_entry(range, entry) {
    // console.log(range, entry);
    // [0, 20]
    // [50, 10, 5]
    let data = [];
    let pos = range[0];
    let left_Length = range[1];

    console.log(pos, left_Length, entry);

    if (pos < entry[1]) {
        // calculates the difference
        let high = entry[1] - range[0];
        if (high > range[1]) {
            high = range[1];
        }

        data.push([range[0], high]);

        pos = range[0] + high + 1;
        left_Length = range[1] - pos;
    } else {
        data.push([]);
    }

    if (pos >= entry[1]) {
        let high = entry[2] - 1;
        if (entry[2] > left_Length) {
            high = left_Length;
        }

        let convert = pos - entry[1];
        convert = entry[0] + convert;

        data.push([convert, high]);

        pos = pos + high + 1;
        left_Length = range[1] - pos;
    } else {
        data.push([]);
    }

    if (left_Length > 0) {
        data.push([pos, left_Length]);
    } else {
        data.push([]);
    }

    return data;
}