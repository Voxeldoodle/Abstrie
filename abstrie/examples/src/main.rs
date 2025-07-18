// Usage examples and benchmarks
use abstrie_core::GeneralizationTrie;

fn main() {
    // Character trie example
    let mut char_trie = GeneralizationTrie::new();
    let words = vec!["cat", "car", "dog", "dot"];
    for w in &words {
        char_trie.insert(&w.chars().collect::<Vec<_>>());
    }
    println!("Character Trie:\n{}", char_trie.visualize_tree());

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
    println!("Word Trie:\n{}", word_trie.visualize_tree());
}
