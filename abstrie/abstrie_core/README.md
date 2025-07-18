# abstrie_core

Core framework crate for Abstrie: a high-performance, generic Rust trie for pattern abstraction and generalization.

## Features
- Generic trie over any `T: Clone + Eq + Hash`
- Efficient insertion and merging of sequences
- Visualize trie structure as a classic tree
- Minimal, extensible core for pluggable abstraction strategies

## Status
- [x] Core data structures (`GeneralizationTrie`, `TrieNode`, `Pattern`)
- [x] Insertion for generic sequences
- [x] Tree visualization (`visualize_tree`)
- [x] Minimal `Display` implementation
- [x] Unit tests for char and word tries
- [ ] Node merging and pattern extraction
- [ ] Abstraction strategy trait implementations
- [ ] Performance optimizations

## Usage
Add to your workspace Cargo.toml:

```toml
[dependencies]
abstrie_core = { path = "../abstrie_core" }
```

Basic example:

```rust
use abstrie_core::GeneralizationTrie;

// Character trie
let mut trie = GeneralizationTrie::new();
for word in ["cat", "car", "dog", "dot"] {
    trie.insert(&word.chars().collect::<Vec<_>>());
}
println!("{}", trie.visualize_tree());

// Word trie
let mut trie = GeneralizationTrie::new();
for sentence in [vec!["the", "cat"], vec!["the", "dog"], vec!["a", "dog"]] {
    trie.insert(&sentence);
}
println!("{}", trie.visualize_tree());
```

## Project Goals
- Zero-cost abstractions, static dispatch
- Pluggable generalization strategies
- Deterministic, reproducible pattern generation
- Memory and computational efficiency

See the main project requirements for more details.
