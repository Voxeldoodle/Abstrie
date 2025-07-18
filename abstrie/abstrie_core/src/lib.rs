// abstrie_core: Core framework crate
use std::collections::HashMap;
use std::hash::Hash;

pub struct TrieNode<T: Clone + Eq + Hash> {
    pub children: HashMap<T, TrieNode<T>>,
    pub is_terminal: bool,
    // Add more fields as needed (e.g., frequency, metadata)
}

impl<T: Clone + Eq + Hash> TrieNode<T> {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_terminal: false,
        }
    }

    pub fn insert(&mut self, sequence: &[T]) {
        let mut node = self;
        for token in sequence {
            node = node.children.entry(token.clone()).or_insert_with(TrieNode::new);
        }
        node.is_terminal = true;
    }

    pub fn visualize_tree<F>(&self, f: &mut F, prefix: String, is_last: bool, label: Option<&T>)
    where
        F: std::fmt::Write,
        T: std::fmt::Display,
    {
        let connector = if prefix.is_empty() { "" } else if is_last { "└── " } else { "├── " };
        if let Some(l) = label {
            let _ = write!(f, "{}{}{}{}\n", prefix, connector, l, if self.is_terminal { " *" } else { "" });
        }
        let child_count = self.children.len();
        let mut i = 0;
        for (k, v) in self.children.iter() {
            i += 1;
            let is_last_child = i == child_count;
            let new_prefix = if prefix.is_empty() {
                String::new()
            } else if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            v.visualize_tree(f, new_prefix, is_last_child, Some(k));
        }
    }
}

pub struct GeneralizationTrie<T: Clone + Eq + Hash> {
    pub root: TrieNode<T>,
    // Add more fields as needed (e.g., node count, config)
}

impl<T: Clone + Eq + Hash> GeneralizationTrie<T> {
    pub fn new() -> Self {
        GeneralizationTrie {
            root: TrieNode::new(),
        }
    }
    pub fn insert(&mut self, sequence: &[T]) {
        self.root.insert(sequence);
    }

    pub fn visualize_tree(&self) -> String
    where
        T: std::fmt::Display,
    {
        let mut s = String::new();
        self.root.visualize_tree(&mut s, String::new(), true, None);
        s
    }
}
use std::fmt;

impl<T: Clone + Eq + Hash + fmt::Display> fmt::Display for TrieNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut keys: Vec<_> = self.children.keys().collect();
        keys.sort_by(|a, b| format!("{}", a).cmp(&format!("{}", b)));
        write!(f, "[{}]", keys.iter().map(|k| format!("{}", k)).collect::<Vec<_>>().join(", "))
    }
}

pub struct Pattern<T: Clone + Eq + Hash> {
    pub sequence: Vec<T>,
    // Add more fields as needed (e.g., frequency, pattern type)
}

pub trait AbstractionStrategy<T: Clone + Eq + Hash> {
    type Config: Default;
    type Pattern;

    fn should_merge(&self, node_a: &TrieNode<T>, node_b: &TrieNode<T>) -> bool;
    fn merge_nodes(&self, nodes: &[TrieNode<T>]) -> TrieNode<T>;
    fn extract_pattern(&self, path: &[T]) -> Self::Pattern;
}
