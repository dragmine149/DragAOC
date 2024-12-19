/** @type {Document} */
const doc = eval('document');

/** @param {NS} ns */
export async function main(ns) {
    let code = `
    function test_d() {
        console.log('e')

        ${'$'}.ajax({
            url: 'https://adventofcode.com/2023/day/3',
            crossDomain: true,
            headers: {
                'User-Agent': 'AOC test from bitburner by dan@thebanners.uk',
                'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8',
                'Accept-Language': 'en-GB,en;q=0.5',
                'Accept-Encoding': 'gzip, deflate, br',
                'DNT': '1',
                'Connection': 'keep-alive',
                'Cookie': 'session=53616c7465645f5fac8f5b7ad5506f9c1d0fd78a3d73a64b9a2596d5d668718cc211fd9cbc541bbc64b3bef6f4fbbf6d225504de8c99f21c31daca57b25a2da6',
                'Upgrade-Insecure-Requests': '1',
            }
        }).done(function(response) {
            console.log(response);
        });
    }
    test_d();
    `
    let tag = doc.createElement('script');
    tag.id = 'rtest';
    tag.innerText = code;
    doc.head.appendChild(tag);
}


/**
 * @param {string} html
 * HTML code to show in the terminal
 * @param {string} parent
 * ID of the parent node to place the html code their instead
 */
export function terminalInsert(html, parent = '') {
    if (parent.length > 0) {
        doc.getElementById(parent).innerHTML = html;
        return;
    }
    doc.getElementById("terminal").insertAdjacentHTML('beforeend', `<li>${html}</li>`);
}
