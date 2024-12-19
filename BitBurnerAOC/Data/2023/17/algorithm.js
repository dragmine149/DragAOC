const DIRECTION = Object.freeze({
    UP: 0,
    LEFT: 1,
    DOWN: 2,
    RIGHT: 3
});

/**
 * Runs Dijkstra algorithm (or at leasts attempts to)
 * Credit: https://www.youtube.com/watch?v=pSqmAO-m7Lk
 * 
 * @param {function} edge_func 
 * @param {Number} n size of the graph
 * @param {Number} s starting node
 */
export function dijkstra(edge_func, n, s = [0, 0]) {
    // these are starting values
    let node_list = [];
    for (let i = 0; i < n; i++) {
        node_list.push(new Node());
    }
    node_list[convert_pos_index(s, n)].set_dist(0, null, DIRECTION.RIGHT);

    // Special queue set up to order by shortest path per node
    let pq = new PriorityQueue((a, b) => a[1] < b[1]); // sorts by minimum value between two nodes
    pq.push([convert_pos_index(s, n), 0]); // adding start node

    while (pq.size() > 0) {
        let [index, minValue] = pq.pop();
        // if (node_list[index].visited()) {
        //     continue; // already visited, no need to visit again
        // }
        console.log('a');
        // vis[index] = true;
        if (node_list[index].get_dist() < minValue) {
            continue; // If the value of the current cell is less than the value of current path, we just skip as it's a different branch
        }

        // loop though all edges
        edge_func(convert_index_pos(index), node_list[index].get_dir()).forEach((edge, edx, dir) => {
            if (node_list[index].dir_limit_reached(3, dir)) {
                // this is due to the turn limit being reached so we can't go down this path anyway.
                return;
            }

            // edge is the weight of the next node
            // edx is the index of the edge cell
            let newDist = node_list[index].get_dist() + edge;
            // if the distance from the current cell + the weight of the next cell is less than the current value of the next cell
            if (newDist < node_list[edx].get_dist()) {
                console.log(newDist, index, dir);
                node_list[edx].set_dist(newDist, node_list[index], dir);

                pq.push([edx, newDist]); // to save space and time, we only push if we updated it.
            }
        })
    }
    console.log(node_list);
    // return the last element
    return node_list[n - 1];
}

/**
 * @param {[Number, Number]} pos
 * @param {Number} size
 */
function convert_pos_index(pos, size) {
    let x = pos[0] + 1;
    let y = pos[1] + 1;
    return (y * size) + x - size - 1;
}

/**
 * @param {Number} index
 * @param {Number} size
 */
function convert_index_pos(index, size) {
    let m = index * size;
    let div = Math.floor(index / size);
    let reminder = index % size;

    return [reminder, div];
}

export class Node {
    #dist = Number.MAX_VALUE;
    #prev = null;
    #visited = false;
    #prevDir = null;

    get_dist() {
        return this.#dist;
    }

    get_dir() {
        return this.#prevDir;
    }

    set_dist(new_dist, prev_node, prev_dir) {
        this.#visited = true;
        if (new_dist < this.#dist) {
            this.#dist = new_dist;
            this.#prev = prev_node;
            this.#prevDir = prev_dir;
        }
    }

    visited() {
        return this.#visited;
    }

    dir_limit_reached(limit=3, direction=this.#prevDir) {
        if (limit == 0) {
            return true; // limit reached
        }

        if (this.#prevDir != direction) {
            // we had a turn, limit not reached
            return false;
        }
        if (this.#prev == null) {
            return false; // not enough information for the 3 cell limit to be reached.
        }

        return this.#prev.dir_limit_reached(limit-1, direction);
    }
}

// CREDIT: https://stackoverflow.com/questions/42919469/efficient-way-to-implement-priority-queue-in-javascript/42919752#42919752
// Because i am not making my own piority queue yet at least
const top = 0;
const parent = i => ((i + 1) >>> 1) - 1;
const left = i => (i << 1) + 1;
const right = i => (i + 1) << 1;

export class PriorityQueue {
    constructor(comparator = (a, b) => a > b) {
        this._heap = [];
        this._comparator = comparator;
    }
    size() {
        return this._heap.length;
    }
    isEmpty() {
        return this.size() == 0;
    }
    peek() {
        return this._heap[top];
    }
    push(...values) {
        values.forEach(value => {
            this._heap.push(value);
            this._siftUp();
        });
        return this.size();
    }
    pop() {
        const poppedValue = this.peek();
        const bottom = this.size() - 1;
        if (bottom > top) {
            this._swap(top, bottom);
        }
        this._heap.pop();
        this._siftDown();
        return poppedValue;
    }
    replace(value) {
        const replacedValue = this.peek();
        this._heap[top] = value;
        this._siftDown();
        return replacedValue;
    }
    _greater(i, j) {
        return this._comparator(this._heap[i], this._heap[j]);
    }
    _swap(i, j) {
        [this._heap[i], this._heap[j]] = [this._heap[j], this._heap[i]];
    }
    _siftUp() {
        let node = this.size() - 1;
        while (node > top && this._greater(node, parent(node))) {
            this._swap(node, parent(node));
            node = parent(node);
        }
    }
    _siftDown() {
        let node = top;
        while (
            (left(node) < this.size() && this._greater(left(node), node)) ||
            (right(node) < this.size() && this._greater(right(node), node))
        ) {
            let maxChild = (right(node) < this.size() && this._greater(right(node), left(node))) ? right(node) : left(node);
            this._swap(node, maxChild);
            node = maxChild;
        }
    }
}