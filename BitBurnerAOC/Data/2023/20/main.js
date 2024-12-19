/** @param {NS} ns */
export async function main(ns) {
    // load the data
    // acceptable auguments: ['r', 'd', 'd2', 't'];
    // 'r' = real, 'd' = deafult (part1/part2), 'd2' = default (part2 will do part1 if no part2 found), 't' = test (custom data)
    let data = ns.read(`AOC/Data/2023/20/${ns.args[0]}.txt`);
    let game_data = data.split('\n');
    // remove empty entries from the data. (line has to be completly blank)
    game_data = game_data.filter((v) => v != '');
}

function broadcast() {
    // loop though all inputs and send a "low" signal (0)
}

class FlipFlop {
    #state = 0;

    get_state() {
        return this.#state;
    }

    /**
     * @param {Number} signal
     */
    set_state(signal) {
        this.#state = signal == 0 ? (this.#state == 0 ? 1 : 0) : this.#state;
        
        // return the opposite 
        if (signal == 0) {
            return this.#state;
        }
    }
}

class Conjunction {
    #memory = 0;

    /**
     * @param {Number} pulse
     */
    recieve_pulse(pulse) {
        this.#memory = pulse;
        return (this.#memory == 0 ? 1 : 0);
    }

    get_memory() {
        return this.#memory;
    }
}