use abstrie_core::{GeneralizationTrie};

#[test]
fn test_char_trie_visualization() {
    let mut trie = GeneralizationTrie::new();
    let words = vec!["cat", "car", "dog", "dot"];
    for w in &words {
        trie.insert(&w.chars().collect::<Vec<_>>());
    }
    let vis = trie.visualize_tree();
    // Check for expected structure
    assert!(vis.contains("c"));
    assert!(vis.contains("d"));
    assert!(vis.contains("a"));
    assert!(vis.contains("o"));
    assert!(vis.contains("t *")); // terminal nodes
}

#[test]
fn test_word_trie_visualization() {
    let mut trie = GeneralizationTrie::new();
    let sentences = vec![
        vec!["the", "cat"],
        vec!["the", "dog"],
        vec!["a", "dog"],
    ];
    for s in &sentences {
        trie.insert(s);
    }
    let vis = trie.visualize_tree();
    assert!(vis.contains("the"));
    assert!(vis.contains("cat *"));
    assert!(vis.contains("dog *"));
    assert!(vis.contains("a"));
}
