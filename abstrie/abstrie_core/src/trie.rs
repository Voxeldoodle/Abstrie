//! Core trie data structures and algorithms
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct PrefixNode<T: Clone + Eq + Hash + Debug> {
    pub prefix_length : usize,
    pub prefixes : HashSet<Vec<T>>,
    pub children: HashMap<T, PrefixNode<T>>,
    pub is_terminal: bool,
}

impl<T: Clone + Eq + Hash + Debug> PrefixNode<T> {
    pub fn new_with_prefixes(prefixes: HashSet<Vec<T>>) -> Self {
        // assert all prefixes have the same length
        assert!(!prefixes.is_empty(), "Prefixes cannot be empty");
        assert!(prefixes.iter().all(|p| p.len() == prefixes.iter().next().unwrap().len()), "All prefixes must have the same length");
        // Create a new PrefixNode with the provided prefixes
        PrefixNode {
            prefix_length: prefixes.iter().next().map_or(0, |p| p.len()),
            prefixes,
            children: HashMap::new(),
            is_terminal: false,
        }
    }

    pub fn new_with_length(length: usize) -> Self {
        // Create a new PrefixNode with the specified length
        PrefixNode {
            prefix_length: length,
            prefixes: HashSet::new(),
            children: HashMap::new(),
            is_terminal: false,
        }
    }

    pub fn add_prefix(&mut self, prefix: Vec<T>) {
        // Add a prefix to the node
        assert!(prefix.len() == self.prefix_length, "Prefix length must match node's prefix length");
        self.prefixes.insert(prefix);
    }
}

impl<T: Clone + Eq + Hash + Debug> PartialEq for PrefixNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.prefix_length == other.prefix_length
        // Uncomment for full equality:
        // && self.prefixes == other.prefixes
    }
}

impl<T: Clone + Eq + Hash + Debug> Eq for PrefixNode<T> {}

impl<T: Clone + Eq + Hash + Debug> Hash for PrefixNode<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.prefix_length.hash(state);
    }
}

impl<T: Debug + Clone + Eq + Hash> Display for PrefixNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.prefixes.is_empty() {
            return write!(f, "{} {{}}", self.prefix_length);
        }

        let mut total = 0;
        let mut parts = Vec::new();

        for p in &self.prefixes {
            let part = format!("{:?}", p);
            total += part.len();
            if total > 100 {
                parts.push("...".to_string());
                break;
            }
            parts.push(part);
        }

        write!(f, "{} {{ {} }}", self.prefix_length, parts.join(", "))
    }
}


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
            node = node
                .children
                .entry(token.clone())
                .or_insert_with(TrieNode::new);
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

    fn update_key(
        prefix_tree: &mut HashMap<PrefixNode<T>, PrefixNode<T>>,
        node_to_check: &PrefixNode<T>,
        prefix: &Vec<T>,
    ) 
    where
        T: Clone + Eq + Hash + Debug,
    {
        let matching_keys: Vec<PrefixNode<T>> = prefix_tree
            .keys()
            .filter(|k| k.prefix_length == node_to_check.prefix_length)
            .cloned()
            .collect();

        for key in matching_keys {
            if let Some(mut node) = prefix_tree.remove(&key) {
                node.add_prefix(prefix.clone());
                prefix_tree.insert(node.clone(), node);
            }
        }
    }
    
    pub fn get_prefixes_tree(
        &self,
    ) -> HashMap<PrefixNode<T>, PrefixNode<T>>
    where 
        T: Clone + Eq + Hash + Debug,
    {
        let mut lengths = HashMap::new();

        // initialize the stack with a tuple containing the root node and an empty prefix
        let mut stack: Vec<(&TrieNode<T>, Vec<T>)> = vec![(&self.root, Vec::new())];
        let mut root_stack: Vec<Vec<T>> = vec![Vec::new()];

        while let Some((node, prefix)) = stack.pop() {
            // Check if this is a branch point (has multiple children) and not the root
            if !prefix.is_empty() && node.children.len() > 1 {
                // Get the ancestor prefix
                let mut pre = root_stack.pop().unwrap();
                while !prefix.starts_with(&pre) {
                    pre = root_stack.pop().unwrap();
                }

                // Process each prefix in the root_stack to build the tree
                let mut tmp_l = &mut lengths;
                for r in root_stack.iter().skip(1) {
                    let l_r = PrefixNode::new_with_length(r.len());
                    
                    // If we haven't seen this length before, create new node
                    if !tmp_l.contains_key(&l_r) {
                        println!("Creating new node for prefix: {:?}", r);
                        let mut new_node = PrefixNode::new_with_length(r.len());
                        new_node.add_prefix(r.clone());
                        tmp_l.insert(l_r.clone(), l_r.clone());
                    }
                    
                    // Get or create the child map
                    if let Some(existing_node) = tmp_l.get_mut(&l_r) {
                        // Add the prefix to the existing node
                        existing_node.add_prefix(r.clone());
                        
                        // Create child map if it's a leaf
                        if existing_node.children.is_empty() {
                            let child_node = PrefixNode::new_with_length(prefix.len());
                            existing_node.children.insert(r[0].clone(), child_node);
                        }
                    }
                }
                
                // Handle the immediate ancestor (pre)
                if !pre.is_empty() {
                    let n_pre = PrefixNode::new_with_length(pre.len());
                    
                    // Get or create node for the ancestor
                    if let Some(existing_node) = tmp_l.get(&n_pre) {
                        // If it exists, add current prefix to its children
                        let mut new_node = existing_node.clone();
                        let prefix_node = PrefixNode::new_with_prefixes([prefix.clone()].into_iter().collect());
                        new_node.children.insert(prefix[0].clone(), prefix_node);
                        tmp_l.insert(n_pre, new_node);
                    } else {
                        // Create new node with current prefix as child
                        let mut new_node = PrefixNode::new_with_length(pre.len());
                        new_node.add_prefix(pre.clone());
                        let prefix_node = PrefixNode::new_with_prefixes([prefix.clone()].into_iter().collect());
                        new_node.children.insert(prefix[0].clone(), prefix_node);
                        tmp_l.insert(n_pre, new_node);
                    }
                } else {
                    // Add the prefix directly to the root level
                    let prefix_node = PrefixNode::new_with_prefixes([prefix.clone()].into_iter().collect());
                    let n_prefix = PrefixNode::new_with_length(prefix.len());
                    if !tmp_l.contains_key(&n_prefix) {
                        tmp_l.insert(n_prefix.clone(), prefix_node);
                    }
                }
                
                // Update stacks for next iteration
                root_stack.push(pre);
                root_stack.push(prefix.clone());
            }
            
            // Add children to the stack for processing
            for (k, child) in &node.children {
                let mut next_prefix = prefix.clone();
                next_prefix.push(k.clone());
                stack.push((child, next_prefix));
            }
        }
        
        lengths
    }
}

impl<T: Clone + Eq + Hash + fmt::Display> fmt::Display for TrieNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut keys: Vec<_> = self.children.keys().collect();
        keys.sort_by(|a, b| format!("{}", a).cmp(&format!("{}", b)));
        write!(
            f,
            "[{}]",
            keys.iter()
                .map(|k| format!("{}", k))
                .collect::<Vec<_>>()
                .join(", ")
        )
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
