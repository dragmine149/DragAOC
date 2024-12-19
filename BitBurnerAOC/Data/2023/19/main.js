class workflow {
    /** @type {{type:String, symbol:String, num:Number, flow:String}[]} */
    #checks = [];
    #failed = '';
    #name = '';

    /** @param {String} workflow */
    constructor(workflow) {
        this.#name = workflow.substring(0, workflow.indexOf('{'));

        workflow = workflow.replace(`${this.#name}{`, '');
        workflow = workflow.replace('}', '');
        
        let info = workflow.split(',');

        for (let ruleIdx = 0; ruleIdx < info.length - 1; ruleIdx++) {
            // Useful tool: https://regex101.com/
            const regex = /(?<type>[xmas])(?<sym>[<>])(?<num>\d+):(?<flow>\w+)/g;

            let rule = info[ruleIdx];
            let regInfo = regex.exec(rule);
            // console.log(rule, regInfo);
            let ruleInfo = {
                'type': regInfo.groups.type,
                'symbol': regInfo.groups.sym,
                'num': Number(regInfo.groups.num),
                'flow': regInfo.groups.flow
            };
            this.#checks.push(ruleInfo);
        }

        this.#failed = info[info.length - 1];
    }

    /** @param {{x:Number, m:Number, a:Number, s:Number}} gear */
    proccess_rules(gear) {
        let result = '';
        for (let checkID = 0; checkID < this.#checks.length; checkID++) {
            let check = this.#checks[checkID];
            if (check.symbol == '<') {
                result = gear[check.type] < check.num ? check.flow : '';
            }
            if (check.symbol == '>') {
                result = gear[check.type] > check.num ? check.flow : '';
            }

            if (result != '') {
                return result;
            }
        }

        return this.#failed;
    }

    get_name() {
        return this.#name;
    }
}

/** @param {NS} ns */
export async function main(ns) {
    // load the data
    // acceptable auguments: ['r', 'd', 'd2', 't'];
    // 'r' = real, 'd' = deafult (part1/part2), 'd2' = default (part2 will do part1 if no part2 found), 't' = test (custom data)
    let data = ns.read(`AOC/Data/2023/19/${ns.args[0]}.txt`);
    let game_data = data.split('\n');
    // line is splitter
    let workflows = [];
    let parts = [];
    let in_part = false;
    game_data.forEach((line) => {
        if (line == '') {
            in_part = true;
            return;
        }

        if (!in_part) {
            workflows.push(line);
            return
        }

        parts.push(line);
        return;
    })

    workflows = process_workflows(workflows);
    console.log(workflows);
    let gears = gear_processor(parts);
    // console.log(gears);
    let accepted = process_gears(gears, workflows);
    // console.log(accepted);

    
    let sum = 0;
    accepted.forEach((gear) => {
        let gearSum = gear.x + gear.m + gear.a + gear.s;
        sum += gearSum;
    })

    ns.tprint(`RESULT: ${sum}`);
}

/** @param {String[]} workflows */
function process_workflows(workflows) {
    let elfflows = {};
    workflows.forEach((flow) => {
        // px{a<2006:qkq,m>2090:A,rfg}
        let cFlow = new workflow(flow);
        elfflows[cFlow.get_name()] = cFlow;
        // let flowName = flow.substring(0, flow.indexOf('{'));
        // elfflows[flowName] = {};

        // flow = flow.replace(`${flowName}{`, '');
        // flow = flow.replace('}', '');

        // let flowInfo = flow.split(',');
        // flowInfo.forEach((section) => {
        //     if (section[1] == '>' || section[1] == '<') {
        //         // elfflows[flowName][section[0]] = section.substring(1);
        //         let info = section.substring(2).split(':');
        //         let result = info[1];
        //         let num = Number(info[0]); // TODO: NaN

        //         elfflows[flowName][section[0]] = {
        //             operation: section[1],
        //             check: num,
        //             result: result
        //         };
        //     }

        //     elfflows[flowName]['failed'] = section;
        // })
    });

    return elfflows;
}

/**
 * @param {Number} gearInfo
 * @param {{
 * operation:String,
 * check:Number,
 * result:String
 * }} data
 */
function process_location(gearInfo, data) {
    if (data.operation == '>') {
        return gearInfo > data.check ? data.result : '';
    }

    if (data.operation == '<') {
        return gearInfo < data.check ? data.result : '';
    }
}

/**
 * @param {String[]} gears
 */
function gear_processor(gears) {
    let newGears = [];
    gears.forEach((gear) => {
        gear = gear.replace('{', '');
        gear = gear.replace('}', '');
        let gearInfo = gear.split(',');
        let gearObj = {};
        gearInfo.forEach((inf) => {
            gearObj[inf[0]] = Number(inf.substring(2));
        })

        newGears.push(gearObj);
    })

    return newGears;
}

/**
 * @param {{x:Number, m:Number, a:Number, s:Number}[]} gears
 * @param {{}} workflows
 */
function process_gears(gears, workflows) {
    let accepted = [];
    gears.forEach((gear) => {
        let location = 'in';
        while (!['R', 'A'].includes(location)) {
        // while (location != 'R' && location != 'A') {
            // let result = '';
            location = workflows[location].proccess_rules(gear);
            // console.log(gear, location, workflows[location]);
            // if (workflows[location]['x'] != undefined) {
            //     result = process_location(gear.x, workflows[location]['x']);
            // }
            // if (result != '') {
            //     location = result;
            //     continue;
            // }

            // if (workflows[location]['m'] != undefined) {
            //     result = process_location(gear.m, workflows[location]['m']);
            // }
            // if (result != '') {
            //     location = result;
            //     continue;
            // }

            // if (workflows[location]['a'] != undefined) {
            //     result = process_location(gear.a, workflows[location]['a']);
            // }
            // if (result != '') {
            //     location = result;
            //     continue;
            // }

            // if (workflows[location]['s'] != undefined) {
            //     result = process_location(gear.s, workflows[location]['s']);
            // }
            // if (result != '') {
            //     location = result;
            //     continue;
            // }

            // location = workflows[location]['failed'];
        }

        if (location == 'A') {
            accepted.push(gear);
        }
    })

    return accepted;
}