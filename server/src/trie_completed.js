class Node {
    constructor () {
        this.children = {};
        this.word = false;
    }
}

class Trie {
    constructor () {
        this.root = new Node();
    }

    insert (word) {
        let current_node = this.root;

        for (let ch of word) {
            // insert new node
            if (!current_node.children[ch]) {
                current_node.children[ch] = new Node();
            }

            current_node = current_node.children[ch];
        }
        
        current_node.word = true;
    }


    search(word) {
        // steps:
        // traverse as far as the substring allows
        // then return all branches

        let current_node = this.root;
        let result = [];

        for (let ch of word) {
            if (!current_node.children[ch]) return [];
            current_node = current_node.children[ch];
        }

        // initialize a stack variable
        // the stack is going to keep a list of the prefixes at each node
        let stack = [[current_node, word]];

        while (stack.length > 0) {
            let [curr, pref] = stack.pop();

            if (curr.word) {
                result.push(pref);
            }

            for (let ch of Object.keys(curr.children)) {
                stack.push([curr.children[ch], pref + ch]);
            }
        }

        return result;
    }
}