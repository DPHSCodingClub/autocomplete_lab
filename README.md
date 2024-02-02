# Autocomplete Lab
Exploring the inner workings of autocomplete

# Installation

Clone repository to working directory
```console
$ git clone https://github.com/DPHSCodingClub/autocomplete_lab.git
$ cd autocomplete_lab
```
<br>

Install node requirements

```console
$ npm install
```

<br>


# Getting Started

To setup the server, `server/src/index.js` is the code which displays searches on the frontend. Currently, script files are added to `server/src/index.html` depending on if you need them. For example, the current script setup is this:

```html
    <script src="naive.js"></script>
    <script src="trie.js"></script>
    <script src="index.js"></script>
```

This is to allow the index file to access the `Trie` object.

# Resources

- Trie article [(Wikipedia)](https://en.wikipedia.org/wiki/Trie)
- Trie implementation in C [(GitHub)](https://github.com/dcjones/hat-trie)
- Using Patricia Merkle Tries in Ethereum [(Article)](https://ethereum.org/developers/docs/data-structures-and-encoding/patricia-merkle-trie)
    - Understanding the Ethereum Trie [(Article)](https://easythereentropy.wordpress.com/2014/06/04/understanding-the-ethereum-trie/)
- Base-16 Modified Merkle Tree [(GitHub)](https://github.com/paritytech/trie)
- Radix Tree implementation in C [(GitHub)](https://github.com/antirez/rax)


# License
[MIT](LICENSE)

# Author
[Joey Malvinni](https://github.com/joeymalvinni)

