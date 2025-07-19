// Usage examples and benchmarks
use abstrie_core::{prelude::*, visualization::print_prefix_tree};

fn main() {
    // Character trie example
    let mut char_trie = GeneralizationTrie::new();
    let words = vec!["cat", "car", "dog", "dot"];
    for w in &words {
        char_trie.insert(&w.chars().collect::<Vec<_>>());
    }
    println!("Character Trie:\n{}", char_trie.print_tree(""));

    // Word trie example
    let mut word_trie = GeneralizationTrie::new();
    let sentences = vec![
        vec!["the", "cat"],
        vec!["the", "dog"],
        vec!["a", "dog"],
    ];
    for s in &sentences {
        word_trie.insert(s);
    }
    println!("Word Trie:\n{}", word_trie.print_tree(" "));

    // Prefix dictionary example
    let mut prefix_trie = GeneralizationTrie::new();
    let words = vec![
        "apple", "app", "application", 
        "banana", "bat", "batman",
        "banner", "banners", "apemon", "apemar"
    ];
    for w in &words {
        prefix_trie.insert(&w.chars().collect::<Vec<_>>());
    }
    println!("\nPrefix Trie Tree:\n{}", prefix_trie.print_tree(""));

    // Get prefixes tree
    let prefixes_tree = prefix_trie.get_prefixes_tree();
    println!("\nRaw Prefixes Tree:\n{:?}", prefixes_tree);
    println!("\nPrefixes Tree:\n{}", print_prefix_tree(&prefixes_tree, ""));


}
