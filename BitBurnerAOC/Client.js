import { log, checkNsInstance, terminalInsert, setNavCommand } from 'AOC/utils.js';

/** @type {Document} */
const doc = eval('document');

export class Client {
    url = "https://adventofcode.com";
    /** @type {NS} */
    ns;
    /** @type {String} */
    #HOST;
    /** @type {Number} */
    #PORT;

    /**
     * @param {NS} ns
     */
    constructor(ns) {
        this.ns = checkNsInstance(ns, "Client");
        // Client secret is stored on the "middle" server

        // Try to load config from file
        if (!this.ns.fileExists('AOC/.config.txt')) {
            ns.write('AOC/.config.txt', JSON.stringify({ 'HOST': 'localhost', 'PORT': 9090 }));
            log(ns, "Missing config file");
            terminalInsert('<div style="color:white; font-size:24px;">Missing config file! A default config file has been generated for you. Please <a id="AOCEdit" style="cursor:pointer; text-decoration:underline">Edit</a> the file to make sure the config is correct!</div>');
            doc.getElementById('AOCEdit').addEventListener('click', setNavCommand.bind(null, 'nano AOC/.config.txt'));
            ns.exit();
        }

        // Load config file
        let config = JSON.parse(ns.read('AOC/.config.txt'));
        this.#HOST = config.HOST;
        this.#PORT = config.PORT;
    }

    // TODO: figure out a good way to get p2, probably have to wait until we have a submit notification.

    /**
     * @param {Number} year
     * @param {Number} day
     */
    async getDay(year, day) {
        return await this.request(`${year}/day/${day}`);
    }

    /**
     * @param {Number} year
     * @param {Number} day
     */
    async getInput(year, day) {
        return await this.request(`${year}/day/${day}/input`);
    }

    /**
     * @param {Number} year
     * @param {Number} day
     * @param {Number} part
     * @param {Number} answer answer has always been a number so far...
     */
    async submitAnswer(year, day, part, answer) {
        return await this.request(`${year}/day/${day}/answer`, new URLSearchParams({
            'level': part,
            'answer': answer
        }));
    }


    /** @param {String} url */
    check_cache(url) {
        log(this.ns, `Checking cache for ${url}`);
        if (this.ns.fileExists(`AOC/Cache/${url}.txt`)) {
            log(this.ns, `Found cache for ${url}`);
            return this.ns.read(`AOC/Cache/${url}.txt`);
        }

        log(this.ns, `No cache found for ${url}`);
        return '';
    }


    /** 
     * @param {String} url 
     */
    async request(url, body = null) {
        var cached = this.check_cache(url);
        if (cached != '') {
            return cached;
        }
        this.ns.tprint(body);
        this.ns.tprint(url);

        let result;
        if (body != null) {
            result = await fetch(
                `http://${this.#HOST}:${this.#PORT}/${url}`, {
                method: "POST",
                body: body
            }
            )
        } else {
            result = await fetch(
                `http://${this.#HOST}:${this.#PORT}/${url}`
            ).catch((r) => {
                this.ns.tprint(`ERROR: Failed to get '${url}' from server with reason ${r}`);
                return '';
            });
        }

        if (result == '') {
            return result;
        }

        log(this.ns, `Recieved response from server for ${result.url}.`);

        if (url.endsWith('.json')) {
            return await result.json();
        }

        let data = await result.text();

        if (url.endsWith('/input')) {
            return data;
        }

        // going to assume most of the stuff is in the `<main>` tags
        let text = data.substring(data.indexOf('<main>'), data.indexOf('</main>') + 1);

        // Replace with the custom div for the scrolling
        text = text.replace('<main>', `<div style="height:1000px;position:relative;"><div style="max-height:100%;overflow:auto;"><div style="height:1500px;">`);
        text = text.replace('</main>', `</div></div></div>`);

        this.ns.write(`AOC/Cache/${url}.txt`, text, 'w');
        return text;
    }
}