use std::collections::{HashMap, BTreeSet, HashSet};
use std::hash::Hash;
use std::fmt::{Debug, Display};

// Generic Trie implementation
#[derive(Debug, Clone)]
pub struct TrieNode<T> {
    children: HashMap<Vec<T>, TrieNode<T>>,
    is_terminal: bool,
}

impl<T> TrieNode<T> 
where 
    T: Clone + Eq + Hash + Debug + Display,
{
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_terminal: false,
        }
    }

    // Build trie with proper segmentation based on branching patterns
    pub fn from_sequences(sequences: &[&[T]]) -> Self {
        Self::build_segmented_trie(sequences, 0)
    }

    fn build_segmented_trie(sequences: &[&[T]], start_pos: usize) -> Self {
        let mut root = TrieNode::new();
        
        if sequences.is_empty() {
            return root;
        }

        // Filter sequences that are long enough
        let valid_sequences: Vec<&[T]> = sequences.iter()
            .filter(|seq| seq.len() > start_pos)
            .map(|seq| *seq)
            .collect();

        if valid_sequences.is_empty() {
            return root;
        }

        // Check if any sequence ends at this position
        for seq in sequences {
            if seq.len() == start_pos {
                root.is_terminal = true;
                break;
            }
        }

        // Find the longest common prefix from current position
        let common_prefix_len = Self::find_longest_common_prefix(&valid_sequences, start_pos);
        
        if common_prefix_len > 0 {
            // If there's a common prefix, create segments based on where sequences diverge
            Self::build_with_common_prefix(&mut root, &valid_sequences, start_pos, common_prefix_len);
        } else {
            // No common prefix, group by first element
            Self::build_without_common_prefix(&mut root, &valid_sequences, start_pos);
        }

        root
    }

    fn find_longest_common_prefix(sequences: &[&[T]], start_pos: usize) -> usize {
        if sequences.len() <= 1 {
            return 0;
        }

        let first_seq = sequences[0];
        let mut common_len = 0;

        for i in start_pos..first_seq.len() {
            let element = &first_seq[i];
            
            // Check if all sequences have the same element at this position
            let all_match = sequences.iter().all(|seq| {
                seq.len() > i && seq[i] == *element
            });

            if all_match {
                common_len = i - start_pos + 1;
            } else {
                break;
            }
        }

        common_len
    }

    fn build_with_common_prefix(
        root: &mut TrieNode<T>, 
        sequences: &[&[T]], 
        start_pos: usize, 
        common_prefix_len: usize
    ) {
        // Create the common prefix segment
        let common_segment: Vec<T> = sequences[0][start_pos..start_pos + common_prefix_len].to_vec();
        
        // Group sequences by what comes after the common prefix
        let mut groups: HashMap<Option<T>, Vec<&[T]>> = HashMap::new();
        
        for seq in sequences {
            let next_pos = start_pos + common_prefix_len;
            let next_element = if seq.len() > next_pos {
                Some(seq[next_pos].clone())
            } else {
                None
            };
            
            groups.entry(next_element).or_insert_with(Vec::new).push(seq);
        }

        // If all sequences have the same continuation or no continuation, 
        // extend the common segment
        if groups.len() == 1 {
            let next_pos = start_pos + common_prefix_len;
            let child = Self::build_segmented_trie(sequences, next_pos);
            root.children.insert(common_segment, child);
        } else {
            // Create segments based on divergence points
            Self::build_divergent_segments(root, sequences, start_pos);
        }
    }

    fn build_without_common_prefix(
        root: &mut TrieNode<T>, 
        sequences: &[&[T]], 
        start_pos: usize
    ) {
        // Group sequences by their first element at start_pos
        let mut groups: HashMap<T, Vec<&[T]>> = HashMap::new();
        
        for seq in sequences {
            if seq.len() > start_pos {
                let first_element = seq[start_pos].clone();
                groups.entry(first_element).or_insert_with(Vec::new).push(seq);
            }
        }

        // For each group, find the optimal segment length
        for (first_element, group_sequences) in groups {
            let segment = Self::find_optimal_segment(&group_sequences, start_pos);
            let child = Self::build_segmented_trie(&group_sequences, start_pos + segment.len());
            root.children.insert(segment, child);
        }
    }

    fn build_divergent_segments(
        root: &mut TrieNode<T>, 
        sequences: &[&[T]], 
        start_pos: usize
    ) {
        // Group sequences by their prefixes until they diverge
        let mut groups: HashMap<Vec<T>, Vec<&[T]>> = HashMap::new();
        
        for seq in sequences {
            // Find where this sequence diverges from others
            let segment = Self::find_segment_until_divergence(seq, sequences, start_pos);
            groups.entry(segment).or_insert_with(Vec::new).push(seq);
        }

        for (segment, group_sequences) in groups {
            let child = Self::build_segmented_trie(&group_sequences, start_pos + segment.len());
            root.children.insert(segment, child);
        }
    }

    fn find_segment_until_divergence(
        target_seq: &[T], 
        all_sequences: &[&[T]], 
        start_pos: usize
    ) -> Vec<T> {
        let mut segment_len = 1;
        
        // Start with at least one element
        if target_seq.len() <= start_pos {
            return Vec::new();
        }

        // Extend segment until we find a good breakpoint
        for len in 1..=(target_seq.len() - start_pos) {
            let potential_segment = target_seq[start_pos..start_pos + len].to_vec();
            
            // Check if this is a good breakpoint
            if Self::is_good_segment_breakpoint(&potential_segment, all_sequences, start_pos) {
                segment_len = len;
                break;
            }
            segment_len = len;
        }

        target_seq[start_pos..start_pos + segment_len].to_vec()
    }

    fn find_optimal_segment(sequences: &[&[T]], start_pos: usize) -> Vec<T> {
        if sequences.is_empty() {
            return Vec::new();
        }

        // For now, use a simple heuristic: extend until sequences diverge
        let first_seq = sequences[0];
        let mut segment_len = 1;

        for len in 1..=(first_seq.len() - start_pos) {
            let current_segment = &first_seq[start_pos..start_pos + len];
            
            // Check if all sequences in this group share this prefix
            let all_share_prefix = sequences.iter().all(|seq| {
                seq.len() >= start_pos + len && 
                seq[start_pos..start_pos + len] == *current_segment
            });

            if all_share_prefix {
                segment_len = len;
                
                // Check if extending further would still be shared
                if len < first_seq.len() - start_pos {
                    let extended_len = len + 1;
                    let still_shared = sequences.iter().all(|seq| {
                        seq.len() >= start_pos + extended_len && 
                        seq[start_pos + len] == first_seq[start_pos + len]
                    });
                    
                    if !still_shared {
                        break; // Good breakpoint found
                    }
                }
            } else {
                break;
            }
        }

        first_seq[start_pos..start_pos + segment_len].to_vec()
    }

    fn is_good_segment_breakpoint(
        segment: &[T], 
        all_sequences: &[&[T]], 
        start_pos: usize
    ) -> bool {
        // A segment is a good breakpoint if:
        // 1. Some sequences continue after it
        // 2. Some sequences end at it, or
        // 3. Sequences diverge after it

        let sequences_with_segment: Vec<&[T]> = all_sequences.iter()
            .filter(|seq| {
                seq.len() >= start_pos + segment.len() && 
                seq[start_pos..start_pos + segment.len()] == *segment
            })
            .map(|seq| *seq)
            .collect();

        if sequences_with_segment.len() <= 1 {
            return true;
        }

        // Check if sequences diverge after this segment
        let next_pos = start_pos + segment.len();
        let mut next_elements: HashSet<Option<T>> = HashSet::new();
        
        for seq in sequences_with_segment {
            let next_element = if seq.len() > next_pos {
                Some(seq[next_pos].clone())
            } else {
                None
            };
            next_elements.insert(next_element);
        }

        next_elements.len() > 1
    }

    // Tree visualization method for regular trie
    pub fn print_tree(&self) {
        self.print_tree_with_options(" ", ".", false);
    }

    pub fn print_tree_with_options(&self, separator: &str, terminal_char: &str, quote_elements: bool) {
        println!("Root{}", if self.is_terminal { terminal_char } else { "" });
        self.print_tree_recursive("", true, separator, terminal_char, quote_elements);
    }

    fn print_tree_recursive(&self, prefix: &str, is_last_sibling: bool, separator: &str, terminal_char: &str, quote_elements: bool) {
        let children: Vec<_> = self.children.iter().collect();
        
        for (i, (segment, child)) in children.iter().enumerate() {
            let is_last = i == children.len() - 1;
            let branch = if is_last { "└─" } else { "├─" };
            let child_prefix = if is_last { "  " } else { "│ " };
            
            let segment_display = Self::format_segment(segment, separator, quote_elements);
            println!("{}{} {}{}", prefix, branch, segment_display, 
                if child.is_terminal { terminal_char } else { "" });
            
            child.print_tree_recursive(&format!("{}{}", prefix, child_prefix), is_last, separator, terminal_char, quote_elements);
        }
    }

    fn format_segment(segment: &[T], separator: &str, quote_elements: bool) -> String {
        segment.iter()
            .map(|item| {
                if quote_elements {
                    format!("{:?}", item)
                } else {
                    format!("{}", item)
                }
            })
            .collect::<Vec<_>>()
            .join(separator)
    }
}

// Generic Length-grouped trie implementation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LengthGroupKey<T> 
where 
    T: Clone + Eq + Ord + Hash + Debug,
{
    length: usize,
    segments: BTreeSet<Vec<T>>,
}

impl<T> LengthGroupKey<T> 
where 
    T: Clone + Eq + Ord + Hash + Debug,
{
    pub fn new(length: usize, segments: BTreeSet<Vec<T>>) -> Self {
        LengthGroupKey { length, segments }
    }
}

#[derive(Debug, Clone)]
pub struct LengthGroupedNode<T> 
where 
    T: Clone + Eq + Hash + Debug + std::cmp::Ord,
{
    children: HashMap<LengthGroupKey<T>, LengthGroupedNode<T>>,
    is_terminal: bool,
}

impl<T> LengthGroupedNode<T>
where 
    T: Clone + Eq + Ord + Hash + Debug,
{
    pub fn new() -> Self {
        LengthGroupedNode {
            children: HashMap::new(),
            is_terminal: false,
        }
    }

    pub fn from_trie(trie_root: &TrieNode<T>) -> Self {
        Self::transform_node_recursive(trie_root)
    }

    fn transform_node_recursive(current_node: &TrieNode<T>) -> Self {
        let mut new_node = LengthGroupedNode::new();
        new_node.is_terminal = current_node.is_terminal;

        // If this is a leaf node, return immediately
        if current_node.children.is_empty() {
            return new_node;
        }

        // Group children by segment length
        let mut length_groups: HashMap<usize, BTreeSet<Vec<T>>> = HashMap::new();
        
        for (segment, _) in &current_node.children {
            let segment_length = segment.len();
            length_groups.entry(segment_length)
                .or_insert_with(BTreeSet::new)
                .insert(segment.clone());
        }

        // Process each length group recursively
        for (length, segment_set) in length_groups {
            let group_key = LengthGroupKey::new(length, segment_set.clone());
            
            // Merge all children in this length group
            let merged_child = Self::merge_children_by_length_group(
                &current_node.children,
                &segment_set,
            );
            
            new_node.children.insert(group_key, merged_child);
        }

        new_node
    }

    fn merge_children_by_length_group(
        original_children: &HashMap<Vec<T>, TrieNode<T>>,
        segments_in_group: &BTreeSet<Vec<T>>,
    ) -> Self {
        let mut merged_node = LengthGroupedNode::new();
        
        // Collect all grandchildren from all segments in this group
        let mut all_grandchildren: HashMap<Vec<T>, TrieNode<T>> = HashMap::new();
        let mut any_terminal = false;

        for segment in segments_in_group {
            if let Some(child_node) = original_children.get(segment) {
                // Check if any child in this group is terminal
                if child_node.is_terminal {
                    any_terminal = true;
                }
                
                // Collect all grandchildren
                for (grandchild_segment, grandchild_node) in &child_node.children {
                    all_grandchildren.insert(grandchild_segment.clone(), grandchild_node.clone());
                }
            }
        }

        merged_node.is_terminal = any_terminal;

        // Now group all grandchildren by their segment lengths
        let mut length_groups: HashMap<usize, BTreeSet<Vec<T>>> = HashMap::new();
        
        for (segment, _) in &all_grandchildren {
            let segment_length = segment.len();
            length_groups.entry(segment_length)
                .or_insert_with(BTreeSet::new)
                .insert(segment.clone());
        }

        // Process each length group recursively
        for (length, segment_set) in length_groups {
            let group_key = LengthGroupKey::new(length, segment_set.clone());
            
            // Recursively merge this length group
            let merged_child = Self::merge_children_by_length_group(
                &all_grandchildren,
                &segment_set,
            );
            
            merged_node.children.insert(group_key, merged_child);
        }

        merged_node
    }

    // Original print method (kept for backward compatibility)
    pub fn print(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        
        if self.is_terminal {
            println!("{}Terminal", indent);
        }

        for (key, child) in &self.children {
            let segments: Vec<Vec<T>> = key.segments.iter().cloned().collect();
            println!("{}({}, {:?}):", indent, key.length, segments);
            child.print(depth + 1);
        }
    }

    // Tree visualization method for length-grouped trie
    pub fn print_tree(&self) {
        self.print_tree_with_options(" ", ".");
    }

    pub fn print_tree_with_options(&self, separator: &str, terminal_char: &str) {
        println!("Root{}", if self.is_terminal { terminal_char } else { "" });
        self.print_tree_recursive("", true, separator, terminal_char);
    }

    fn print_tree_recursive(&self, prefix: &str, is_last_sibling: bool, separator: &str, terminal_char: &str) {
        let mut children: Vec<_> = self.children.iter().collect();
        // Sort by length first, then by segments for consistent output
        children.sort_by(|a, b| {
            a.0.length.cmp(&b.0.length)
                .then_with(|| a.0.segments.cmp(&b.0.segments))
        });
        
        for (i, (key, child)) in children.iter().enumerate() {
            let is_last = i == children.len() - 1;
            let branch = if is_last { "└─" } else { "├─" };
            let child_prefix = if is_last { "  " } else { "│ " };
            
            // Format the segments more compactly
            let segments_display = if key.segments.len() == 1 {
                Self::format_segment(key.segments.iter().next().unwrap(), separator)
            } else {
                format!("[{}]", 
                    key.segments.iter()
                        .map(|seg| Self::format_segment(seg, separator))
                        .collect::<Vec<_>>()
                        .join(", "))
            };
            
            println!("{}{}len={} {}{}", 
                prefix, branch, key.length, segments_display,
                if child.is_terminal { terminal_char } else { "" });
            
            child.print_tree_recursive(&format!("{}{}", prefix, child_prefix), is_last, separator, terminal_char);
        }
    }

    fn format_segment(segment: &[T], separator: &str) -> String {
        segment.iter()
            .map(|item| format!("{:?}", item))
            .collect::<Vec<_>>()
            .join(separator)
    }
}

// Helper functions for string-based examples (backward compatibility)
impl TrieNode<char> {
    pub fn from_words(words: &[&str]) -> Self {
        let char_sequences: Vec<Vec<char>> = words.iter()
            .map(|word| word.chars().collect())
            .collect();
        let sequences: Vec<&[char]> = char_sequences.iter()
            .map(|seq| seq.as_slice())
            .collect();
        Self::from_sequences(&sequences)
    }
}