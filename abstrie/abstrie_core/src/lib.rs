// abstrie_core: Core framework crate
use std::collections::HashMap;
use std::fmt::Write;
use std::hash::Hash;

pub struct TrieNode<T: Clone + Eq + Hash> {
    pub children: HashMap<T, TrieNode<T>>,
    pub is_terminal: bool,
    //TODO: Add more fields as needed (e.g., frequency, metadata)
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

    pub fn visualize_tree(&self, token_separator: Option<&str>) -> String
    where
        T: std::fmt::Display,
    {
        use std::collections::VecDeque;
        let token_separator = token_separator.unwrap_or("-");
        let sequence_ender = ".".to_string();
        let mut out_str = String::new();
        let mut stack = VecDeque::new();
        // Each stack entry: (node, level, is_last, label_path, skips)
        // skips: Vec<bool> where skips[i] == true means at that level, the parent was the last child (so no vertical line)
        stack.push_back((&self.root, 0, true, Vec::new(), Vec::new()));
        while let Some((mut node, level, is_last, mut label_path, skips)) = stack.pop_back() {
            // Path compression: collect tokens for linear path
            while node.children.len() == 1 && !node.is_terminal {
                let (k, v) = node.children.iter().next().unwrap();
                label_path.push(k.clone());
                node = v;
            }
            // Build indent prefix using skips
            let mut indent = String::new();
            if level > 0 {
                for (_i, &skip) in skips.iter().enumerate() {
                    if skip {
                        indent.push_str("   ");
                    } else {
                        indent.push_str("│  ");
                    }
                }
                indent.push_str(if is_last { "└── " } else { "├── " });
            }
            // Print only if not root or if root is terminal
            if !label_path.is_empty() || node.is_terminal {
                let label = if !label_path.is_empty() {
                    label_path.iter().map(|t| format!("{}", t)).collect::<Vec<_>>().join(token_separator)
                } else {
                    String::new()
                };
                let _ = write!(out_str, "{}{}{}\n", indent, label, if node.is_terminal { sequence_ender.clone() } else { "".to_string() });
            }
            // Prepare children
            let mut children: Vec<(&T, &TrieNode<T>)> = node.children.iter().collect();
            children.sort_by(|a, b| format!("{}", a.0).cmp(&format!("{}", b.0)));
            let n = children.len();
            for (i, (k, v)) in children.into_iter().enumerate().rev() {
                let is_last_child = i == n - 1;
                // For each child, start a new label_path with just that token
                let mut new_label_path = Vec::new();
                new_label_path.push(k.clone());
                // Update skips: add whether this child is the last at this level
                let mut new_skips = skips.clone();
                new_skips.push(is_last);
                stack.push_back((v, level + 1, is_last_child, new_label_path, new_skips));
            }
        }
        out_str
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
