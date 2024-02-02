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

