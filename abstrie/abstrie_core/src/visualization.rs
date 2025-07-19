use std::{collections::HashMap, fmt::Write};
use crate::trie::*;

/// Helper trait for pretty-printing tree structures
pub trait TreeDisplay {
    fn print_tree(&self, indent: &str) -> String;
}

pub fn print_prefix_tree<T: Clone + Eq + std::hash::Hash + std::fmt::Debug>(
    tree: &HashMap<PrefixNode<T>, PrefixNode<T>>,
    token_separator: &str,
) -> String {
    use std::collections::VecDeque;
    let mut out_str = String::new();
    
    // Sort nodes by prefix length to process them in order
    let mut nodes: Vec<_> = tree.iter().collect();
    nodes.sort_by_key(|(k, _)| k.prefix_length);
    
    // Build level structure
    let mut level_map: HashMap<usize, Vec<(&PrefixNode<T>, &PrefixNode<T>)>> = HashMap::new();
    for (k, v) in nodes {
        level_map.entry(k.prefix_length)
            .or_insert_with(Vec::new)
            .push((k, v));
    }
    
    // Process each level
    let mut levels: Vec<_> = level_map.keys().collect();
    levels.sort();
    
    for (level_idx, &level) in levels.iter().enumerate() {
        if let Some(nodes) = level_map.get(&level) {
            let is_last_level = level_idx == levels.len() - 1;
            
            // Process nodes at this level
            for (idx, (node, _value)) in nodes.iter().enumerate() {
                let is_last_node = idx == nodes.len() - 1;
                
                // Calculate indent
                let mut indent = String::new();
                for _ in 0..level_idx {
                    indent.push_str("│  ");
                }
                
                // Add branch symbol
                if level_idx > 0 {
                    indent.push_str(if is_last_node { "└── " } else { "├── " });
                }
                
                // Print node information
                let prefixes: Vec<_> = node.prefixes.iter()
                    .map(|p| format!("{:?}", p))
                    .collect();
                let _ = writeln!(
                    out_str,
                    "{}len={} prefixes=[{}]",
                    indent,
                    node.prefix_length,
                    prefixes.join(", ")
                );
                
                // Print children if they exist
                if !node.children.is_empty() {
                    let mut child_indent = indent.clone();
                    if !is_last_node {
                        child_indent.push_str("│  ");
                    } else {
                        child_indent.push_str("   ");
                    }
                    
                    let mut children: Vec<_> = node.children.iter().collect();
                    children.sort_by(|a, b| format!("{:?}", a.0).cmp(&format!("{:?}", b.0)));
                    
                    for (child_idx, (key, child)) in children.iter().enumerate() {
                        let is_last_child = child_idx == children.len() - 1;
                        let branch = if is_last_child { "└── " } else { "├── " };
                        let _ = writeln!(
                            out_str,
                            "{}{}{:?} -> len={} prefixes=[{}]",
                            child_indent,
                            branch,
                            key,
                            child.prefix_length,
                            child.prefixes.iter()
                                .map(|p| format!("{:?}", p))
                                .collect::<Vec<_>>()
                                .join(", ")
                        );
                    }
                }
            }
        }
    }
    out_str
}

impl<T: Clone + Eq + std::hash::Hash + std::fmt::Display> TreeDisplay for GeneralizationTrie<T> {
    fn print_tree(&self, token_separator: &str) -> String {
        use std::collections::VecDeque;
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
                let mut new_label_path = Vec::new();
                new_label_path.push(k.clone());
                let mut new_skips = skips.clone();
                new_skips.push(is_last);
                stack.push_back((v, level + 1, is_last_child, new_label_path, new_skips));
            }
        }
        out_str
    }
}
