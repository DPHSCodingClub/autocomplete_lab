use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    word: bool,
    children: HashMap<char, Node>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            word: false,
            children: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Trie {
    root: Node
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: Node::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        
        for ch in word.chars() {
            if !current_node.children.contains_key(&ch) {
                current_node.children.insert(ch, Node::new());
            }

            current_node = current_node.children.get_mut(&ch).unwrap();
        }

        current_node.word = true;
    }

    pub fn search(&mut self, prefix: &str) -> Vec<String> {
        let mut node = &mut self.root;

        for ch in prefix.chars() {
            if !node.children.contains_key(&ch) {
                return Vec::new();
            }

            node = node.children.get_mut(&ch).unwrap();
        }

        let mut result = Vec::new();
        let mut stack = vec![(node, String::from(prefix))];

        while let Some((node, current_word)) = stack.pop() {
            if node.word {
                result.push(current_word.clone());
            }

            for (ch, child_node) in &mut node.children {
                stack.push((
                    child_node,
                    current_word.clone() + &ch.to_string(),
                ));
            }
        }

        result
    }
}


fn small_data(c: &mut Criterion) {
    let binary_file_path = "../data/words.bin";

    let loaded_data: Vec<String> = read_binary_file_into_vector(binary_file_path).expect("can't read binary file");
    let small: Vec<String> = loaded_data[0..10_000].to_vec();

    let mut t = Trie::new();

    for data in &small {
        t.insert(data);
    }

    let mut group = c.benchmark_group("Smaller dataset");
    group.bench_function("trie 'a'", |b| b.iter(|| t.search(black_box("a"))));
    group.bench_function("trie 'b'", |b| b.iter(|| t.search(black_box("b"))));
    group.bench_function("trie 'ab'", |b| b.iter(|| t.search(black_box("ab"))));
    group.bench_function("trie 'pre'", |b| b.iter(|| t.search(black_box("pre"))));
    group.bench_function("trie 'test'", |b| b.iter(|| t.search(black_box("test"))));
    group.bench_function("trie 'practice'", |b| b.iter(|| t.search(black_box("practice"))));

    for size in [10, 100, 1000, 10000].iter() {
        let input_data: Vec<String> = loaded_data.iter().take(*size).cloned().collect();
        let mut trie = Trie::new();

        for data in &input_data {
            trie.insert(data);
        }

        group.bench_function(format!("trie size {}", size).as_str(), |b| {
            b.iter(|| trie.search(black_box("a")))
        }).measurement_time(Duration::from_secs(30));
    }

    group.finish();
}

fn large_data(c: &mut Criterion) {
    let binary_file_path = "../data/words.bin";

    let loaded_data: Vec<String> = read_binary_file_into_vector(binary_file_path).expect("can't read binary file");

    let mut t = Trie::new();

    for data in &loaded_data {
        t.insert(data);
    }

    // testing words on the entire tree
    let mut group = c.benchmark_group("Large dataset");
    group.bench_function("trie 'a'", |b| b.iter(|| t.search(black_box("a"))));
    group.bench_function("trie 'b'", |b| b.iter(|| t.search(black_box("b"))));
    group.bench_function("trie 'ab'", |b| b.iter(|| t.search(black_box("ab"))));

    for size in [10, 100, 1000, 10000].iter() {
        let input_data: Vec<String> = loaded_data.iter().take(*size).cloned().collect();
        let mut trie = Trie::new();

        for data in &input_data {
            trie.insert(data);
        }

        group.bench_function(format!("trie size {}", size).as_str(), |b| {
            b.iter(|| trie.search(black_box("a")))
        }).measurement_time(Duration::from_secs(30));
    }
}


fn read_binary_file_into_vector(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    for line in reader.lines() {
        data.push(line?);
    }

    Ok(data)
}


criterion_group!(benches, small_data, large_data);
criterion_main!(benches);
