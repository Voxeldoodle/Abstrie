use std::fmt::Write;
use crate::trie::*;

/// Helper trait for pretty-printing tree structures
pub trait TreeDisplay {
    fn print_tree(&self, indent: &str) -> String;
}

impl<T: std::fmt::Display + Clone + std::hash::Hash + Eq> PrefixNode<T> {
    pub fn print(&self, dict: &std::collections::HashMap<Vec<T>, Self>, indent: &str, level: usize) {
        let mut entries: Vec<_> = dict.iter().collect();
        entries.sort_by(|a, b| {
            let a_str = a.0.iter().map(|c| format!("{}", c)).collect::<String>();
            let b_str = b.0.iter().map(|c| format!("{}", c)).collect::<String>();
            a_str.cmp(&b_str)
        });
        
        for (key, value) in entries {
            let prefix = key.iter().map(|c| format!("{}", c)).collect::<String>();
            match value {
                PrefixNode::Branch(branches) => {
                    println!("{}├── {} →", indent, prefix);
                    self.print(branches, &format!("{}│   ", indent), level + 1);
                }
                PrefixNode::Terminal => {
                    println!("{}└── {} (terminal)", indent, prefix);
                }
            }
        }
    }
}

impl LengthNode {
    pub fn print(&self, dict: &std::collections::HashMap<usize, Self>, level: usize) {
        let mut entries: Vec<_> = dict.iter().collect();
        entries.sort_by_key(|a| a.0);
        
        let indent = "    ".repeat(level);
        for (key, value) in entries {
            match value {
                LengthNode::Branch(branches) => {
                    println!("{}├── Length {} →", indent, key);
                    self.print(branches, level + 1);
                }
                LengthNode::Terminal => {
                    println!("{}└── Length {} (terminal)", indent, key);
                }
            }
        }
    }
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
