document.addEventListener('DOMContentLoaded', () => {
    // const trie = new Trie();
	const trie = new Naive();

    fetch('words_alpha.json')
        .then(response => response.json())
        .then(data => {
            for (let word of data) {
                trie.insert(word);
            }
        })
        .catch(error => console.error('Error loading data:', error));

    const input = document.getElementById('input');
    const span = document.getElementById('result');
    const time = document.getElementById('time');

    input.addEventListener('input', () => {
        const value = input.value.trim().toLowerCase();

        if (!value.length > 0) return;

        const startTime = performance.now();
        const res = trie.search(value);
        const endTime = performance.now();
        const elapsed = endTime - startTime;

        const final = res.sort((a, b) => a.length - b.length).slice(0, 20);

        let string = "";

        for (let result of final) {
            if (result.length == value.length) string += `<strong>${result}</strong><br>`;
            else string += `<strong>${result.slice(0, value.length)}</strong>${result.slice(value.length)}<br>`
        }

        span.innerHTML = final.length > 0 ? string : 'No results';

        time.innerText = `Time taken: ${elapsed}ms`;
    });
});
