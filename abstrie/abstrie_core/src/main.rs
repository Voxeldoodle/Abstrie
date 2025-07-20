use abstrie_core::trie::{TrieNode, LengthGroupedNode};

fn main() {
    println!("=== Character-based example (original) ===");
    let words = vec!["ape", "app", "application", "bans", "bat", "banner", "pot", "potion"];
    
    println!("Building trie from words: {:?}", words);
    let char_trie = TrieNode::from_words(&words);
    
    println!("\nOriginal trie tree structure:");
    char_trie.print_tree();
    
    println!("\nTransforming to length-grouped trie...");
    let char_length_grouped_trie = LengthGroupedNode::from_trie(&char_trie);
    
    println!("\nLength-grouped trie tree structure:");
    char_length_grouped_trie.print_tree();
    
    println!("\n=== Word-based example ===");
    let sentences = vec![
        &["the", "dog", "ate", "choco"][..],
        &["the", "dog", "ate","cookie"][..],
        &["the", "dog"][..],
        &["a", "big", "dog", "ate", "choco"][..],
        &["a", "cat"][..],
        &["a", "big", "dog", "ate","cookie"][..],
    ];
    
    println!("Building trie from sentences: {:?}", sentences);
    let word_trie = TrieNode::from_sequences(&sentences);
    
    println!("\nWord-based trie tree structure:");
    word_trie.print_tree();
    
    println!("\nTransforming to length-grouped trie...");
    let word_length_grouped_trie = LengthGroupedNode::from_trie(&word_trie);
    
    println!("\nWord-based length-grouped trie tree structure:");
    word_length_grouped_trie.print_tree();
    
    println!("\n=== Integer-based example ===");
    let int_sequences = vec![
        &[1, 2][..],
        &[1, 3][..],
        &[1, 2, 4, 5][..],
        &[2, 3][..],
        &[2, 3, 4][..],
    ];
    
    println!("Building trie from integer sequences: {:?}", int_sequences);
    let int_trie = TrieNode::from_sequences(&int_sequences);
    
    println!("\nInteger-based trie tree structure:");
    int_trie.print_tree();
    
    println!("\nTransforming to length-grouped trie...");
    let int_length_grouped_trie = LengthGroupedNode::from_trie(&int_trie);
    
    println!("\nInteger-based length-grouped trie tree structure:");
    int_length_grouped_trie.print_tree();
}