//! Core trie data structures and algorithms
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

// Helper enums for prefix dictionary
#[derive(Debug, Clone)]
pub enum PrefixNode<T> {
    Branch(std::collections::HashMap<Vec<T>, PrefixNode<T>>),
    Terminal,
}

#[derive(Debug, Clone)]
pub enum LengthNode {
    Branch(std::collections::HashMap<usize, LengthNode>),
    Terminal,
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

    /// Returns two nested HashMaps:
    /// - prefixes: HashMap<Vec<T>, ...> representing the trie structure by prefix
    /// - lengths: HashMap<usize, ...> representing the trie structure by prefix length
    /// Each leaf is marked with a special value (e.g., Terminal)
    pub fn get_prefixes_dict(
        &self,
    ) -> (
        std::collections::HashMap<Vec<T>, PrefixNode<T>>,
        std::collections::HashMap<usize, LengthNode>,
    )
    where
        T: std::fmt::Display + Clone + Eq + std::hash::Hash + std::fmt::Debug,
    {
        use std::collections::HashMap;
        println!("Starting get_prefixes_dict");
        let mut prefixes: HashMap<Vec<T>, PrefixNode<T>> = HashMap::new();
        let mut lengths: HashMap<usize, LengthNode> = HashMap::new();
        let mut stack = vec![(
            &self.root,
            Vec::new(), // prefix
        )];
        let mut root_stack = vec![Vec::<T>::new()]; // Stack to track ancestor prefixes
        while let Some((node, prefix)) = stack.pop() {
            // println!("Processing node with children: {:?}", node.children.keys());
            print!("Processing node with prefix: {:?}\n", prefix);
            if !prefix.is_empty() && node.children.len() > 1 {
                println!("Found branch node with prefix: {:?}", prefix);
                let pre = if let Some(pre) = root_stack.pop() {
                    println!("Popped ancestor prefix: {:?}", pre);
                    pre
                } else {
                    println!("No ancestor prefix found, using empty Vec");
                    Vec::new()
                };
                // Insert into prefixes and lengths
                let mut tmp_p = &mut prefixes;
                let mut tmp_l = &mut lengths;
                println!("Current status: {:?} {:?} {:?}", root_stack, tmp_p, tmp_l);
                // Process root_stack if not empty and has valid prefixes
                if !root_stack.is_empty() {
                    // First, ensure all branches exist
                    for r in &root_stack {
                        if !r.is_empty() {
                            println!("Processing root_stack entry: {:?}", r);
                            let l_r = r.len();
                            if !tmp_p.contains_key(r) {
                                println!("Creating new branch for prefix: {:?}", r);
                                tmp_p.insert(r.clone(), PrefixNode::Branch(HashMap::new()));
                            } else {
                                println!("Found existing node for prefix: {:?}", r);
                                if let Some(PrefixNode::Terminal) = tmp_p.get(r) {
                                    println!("Converting terminal to branch for: {:?}", r);
                                    tmp_p.insert(r.clone(), PrefixNode::Branch(HashMap::new()));
                                }
                            }
                            if !tmp_l.contains_key(&l_r) {
                                let length_map = HashMap::new();
                                tmp_l.insert(l_r, LengthNode::Branch(length_map));
                            } else {
                                if let Some(LengthNode::Terminal) = tmp_l.get(&l_r) {
                                    tmp_l.insert(l_r, LengthNode::Terminal);
                                }
                            }

                            tmp_p = tmp_p
                                .get_mut(r)
                                .and_then(|n| {
                                    if let PrefixNode::Branch(map) = n {
                                        Some(map)
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or_else(|| panic!("Branch not found for {:?}", r));
                            tmp_l = tmp_l
                                .get_mut(&l_r)
                                .and_then(|n| {
                                    if let LengthNode::Branch(map) = n {
                                        Some(map)
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or_else(|| {
                                    panic!("Length branch not found for length {}", l_r)
                                });
                        }
                    }
                }

                // If we have pre
                if !pre.is_empty() {
                    // If tmp[pre] is a branch, insert prefix
                    if let Some(PrefixNode::Branch(map)) = tmp_p.get_mut(&pre) {
                        if !map.contains_key(&prefix) {
                            map.insert(prefix.clone(), PrefixNode::Terminal);
                        }
                    } else {
                        // If pre is not a branch, we need to create it
                        tmp_p.insert(pre.clone(), PrefixNode::Branch(HashMap::new()));
                        if let Some(PrefixNode::Branch(map)) = tmp_p.get_mut(&pre) {
                            map.insert(prefix.clone(), PrefixNode::Terminal);
                        }
                    }
                    if let Some(LengthNode::Branch(map)) = tmp_l.get_mut(&pre.len()) {
                        if !map.contains_key(&prefix.len()) {
                            map.insert(prefix.len(), LengthNode::Terminal);
                        }
                    } else {
                        // If pre is not a branch, we need to create it
                        tmp_l.insert(pre.len(), LengthNode::Branch(HashMap::new()));
                        if let Some(LengthNode::Branch(map)) = tmp_l.get_mut(&pre.len()) {
                            map.insert(prefix.len(), LengthNode::Terminal);
                        }
                    }
                } else {
                    if !tmp_p.contains_key(&prefix) {
                        tmp_p.insert(prefix.clone(), PrefixNode::Terminal);
                    }
                    if !tmp_l.contains_key(&prefix.len()) {
                        tmp_l.insert(prefix.len(), LengthNode::Terminal);
                    }
                }
                root_stack.push(pre);
                root_stack.push(prefix.clone());
                println!("Updated root_stack: {:?}", root_stack);
                println!("Current prefixes tree: {:?}", prefixes);
            }
            // print node children
            println!("Node children: {:?}", node.children.keys());
            for (k, child) in &node.children {
                // if !child.is_terminal {
                    let mut union = prefix.clone();
                    union.push(k.clone());
                    println!("Adding child to stack: {:?}", union);
                    stack.push((child, union));
                // }
            }
        }
        (prefixes, lengths)
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

impl<T: Clone + Eq + Hash> GeneralizationTrie<T> {
    pub fn get_prefixes_dict_by_path(
        &self,
    ) -> (
        std::collections::HashMap<Vec<T>, PrefixNode<T>>,
        std::collections::HashMap<usize, LengthNode>,
    )
    where
        T: std::fmt::Display + Clone + Eq + std::hash::Hash + std::fmt::Debug,
    {
        use std::collections::HashMap;
        println!("Starting get_prefixes_dict_by_path");
        let mut prefixes: HashMap<Vec<T>, PrefixNode<T>> = HashMap::new();
        let mut lengths: HashMap<usize, LengthNode> = HashMap::new();
        let mut stack = vec![(
            &self.root,
            Vec::new(), // prefix
        )];
        let mut root_stack = vec![Vec::<T>::new()]; // Stack to track ancestor prefixes

        while let Some((node, prefix)) = stack.pop() {
            println!("Processing node with prefix: {:?}", prefix);
            
            // Only process nodes that are branch points
            if !prefix.is_empty() && node.children.len() > 1 {
                println!("Found branch node with prefix: {:?}", prefix);
                
                // Get the ancestor prefix
                let pre = root_stack.pop().unwrap_or_else(Vec::new);
                println!("Popped ancestor prefix: {:?}", pre);
                
                // First, handle all ancestors in root_stack
                for r in &root_stack {
                    if !r.is_empty() {
                        // Check and update prefix tree
                        let should_create_prefix_branch = match prefixes.get(r) {
                            Some(PrefixNode::Terminal) | None => true,
                            _ => false,
                        };
                        if should_create_prefix_branch {
                            println!("Creating/updating branch for prefix: {:?}", r);
                            prefixes.insert(r.clone(), PrefixNode::Branch(HashMap::new()));
                        }
                        
                        // Check and update length tree
                        let l_r = r.len();
                        let should_create_length_branch = match lengths.get(&l_r) {
                            Some(LengthNode::Terminal) | None => true,
                            _ => false,
                        };
                        if should_create_length_branch {
                            lengths.insert(l_r, LengthNode::Branch(HashMap::new()));
                        }
                    }
                }
                
                // Handle the immediate ancestor (pre)
                if !pre.is_empty() {
                    // Update prefix tree
                    let should_create_pre_branch = match prefixes.get(&pre) {
                        Some(PrefixNode::Terminal) | None => true,
                        _ => false,
                    };
                    if should_create_pre_branch {
                        prefixes.insert(pre.clone(), PrefixNode::Branch(HashMap::new()));
                    }
                    
                    // Add current prefix as terminal in the ancestor's branch
                    if let Some(PrefixNode::Branch(map)) = prefixes.get_mut(&pre) {
                        map.insert(prefix.clone(), PrefixNode::Terminal);
                    }
                    
                    // Update length tree
                    let pre_len = pre.len();
                    let should_create_length_branch = match lengths.get(&pre_len) {
                        Some(LengthNode::Terminal) | None => true,
                        _ => false,
                    };
                    if should_create_length_branch {
                        lengths.insert(pre_len, LengthNode::Branch(HashMap::new()));
                    }
                    
                    if let Some(LengthNode::Branch(map)) = lengths.get_mut(&pre_len) {
                        map.insert(prefix.len(), LengthNode::Terminal);
                    }
                } else {
                    // Handle root level prefix
                    prefixes.insert(prefix.clone(), PrefixNode::Terminal);
                    lengths.insert(prefix.len(), LengthNode::Terminal);
                }
                
                // Update stacks for next iteration
                root_stack.push(pre);
                root_stack.push(prefix.clone());
                println!("Updated root_stack: {:?}", root_stack);
                println!("Current prefixes tree: {:?}", prefixes);
            }
            
            // Add all children to the stack
            println!("Node children: {:?}", node.children.keys());
            for (k, child) in &node.children {
                let mut union = prefix.clone();
                union.push(k.clone());
                println!("Adding child to stack: {:?}", union);
                stack.push((child, union));
            }
        }
        
        (prefixes, lengths)
    }
}
