use abstrie_core::{GeneralizationTrie};

#[test]
fn test_char_trie_structure() {
    let mut trie = GeneralizationTrie::new();
    let words = vec!["cat", "car", "dog", "dot"];
    for w in &words {
        trie.insert(&w.chars().collect::<Vec<_>>());
    }
    // Check root has 'c' and 'd'
    assert!(trie.root.children.contains_key(&'c'));
    assert!(trie.root.children.contains_key(&'d'));
    // Check 'c' branch
    let c_node = trie.root.children.get(&'c').unwrap();
    assert!(c_node.children.contains_key(&'a'));
    let a_node = c_node.children.get(&'a').unwrap();
    assert!(a_node.children.contains_key(&'t'));
    assert!(a_node.children.contains_key(&'r'));
    // Check terminal
    assert!(a_node.children.get(&'t').unwrap().is_terminal);
    assert!(a_node.children.get(&'r').unwrap().is_terminal);
}

#[test]
fn test_word_trie_structure() {
    let mut trie: GeneralizationTrie<String> = GeneralizationTrie::new();
    let sentences = vec![
        vec!["the", "cat"],
        vec!["the", "dog"],
        vec!["a", "dog"],
    ];
    for s in &sentences {
        let s: Vec<String> = s.iter().map(|w| w.to_string()).collect();
        trie.insert(&s);
    }
    // Check root has 'the' and 'a'
    assert!(trie.root.children.contains_key(&"the".to_string()));
    assert!(trie.root.children.contains_key(&"a".to_string()));
    // Check 'the' branch
    let the_node = trie.root.children.get(&"the".to_string()).unwrap();
    assert!(the_node.children.contains_key(&"cat".to_string()));
    assert!(the_node.children.contains_key(&"dog".to_string()));
    // Check terminal
    assert!(the_node.children.get(&"cat".to_string()).unwrap().is_terminal);
    assert!(the_node.children.get(&"dog".to_string()).unwrap().is_terminal);
}
