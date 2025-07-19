// Usage examples and benchmarks
use abstrie_core::prelude::*;

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

    let (prefixes, lengths) = prefix_trie.get_prefixes_dict();
    // let (prefixes, lengths) = prefix_trie.get_prefixes_dict_by_path();
    
    // Print raw debug representation
    println!("\nRaw Prefix Dictionary:");
    println!("{:#?}", prefixes);
    println!("\nRaw Length Dictionary:");
    println!("{:#?}", lengths);

    println!("\nPrefix Dictionary (branches):");
    PrefixNode::Terminal.print(&prefixes, "", 0);
    println!("\nLength Dictionary (branches):");
    LengthNode::Terminal.print(&lengths, 0);
}
