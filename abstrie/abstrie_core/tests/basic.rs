#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_char_trie() {
        let words = vec!["ape", "app", "application", "bans", "bat", "banner", "pot", "potion"];
        let trie = TrieNode::from_words(&words);
        let length_grouped = LengthGroupedNode::from_trie(&trie);
        
        // Should have 2 main groups: length 2 and length 3
        assert_eq!(length_grouped.children.len(), 2);
    }

    #[test]
    fn test_generic_word_trie() {
        let sentences = vec![
            &["the", "cat"][..],
            &["the", "dog"][..],
            &["a", "cat"][..],
        ];
        
        let trie = TrieNode::from_sequences(&sentences);
        let length_grouped = LengthGroupedNode::from_trie(&trie);
        
        // Should have groups based on word count
        assert!(!length_grouped.children.is_empty());
    }

    #[test]
    fn test_generic_integer_trie() {
        let sequences = vec![
            &[1, 2][..],
            &[1, 3][..],
            &[2, 3][..],
        ];
        
        let trie = TrieNode::from_sequences(&sequences);
        let length_grouped = LengthGroupedNode::from_trie(&trie);
        
        // Should create proper groupings
        assert!(!length_grouped.children.is_empty());
    }

    #[test]
    fn test_tree_visualization() {
        let words = vec!["cat", "car"];
        let trie = TrieNode::from_words(&words);
        
        // Test that print_tree doesn't panic (basic functionality test)
        trie.print_tree();
        trie.print_tree_with_options("-", "*", true);
        
        let length_grouped = LengthGroupedNode::from_trie(&trie);
        length_grouped.print_tree();
        length_grouped.print_tree_with_options("|", "#");
    }
}