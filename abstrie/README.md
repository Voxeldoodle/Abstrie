# Abstrie

A high-performance Rust framework for pattern abstraction and generalization over sequences using trie data structures.

## Project Overview
Abstrie enables extraction of structural patterns from structured or semi-structured data through pluggable generalization strategies. It is designed for:
- **Performance**: Zero-cost abstractions, static dispatch, and optimized token handling
- **Modularity**: Pluggable strategies for different generalization approaches
- **Flexibility**: Character-level and word-level pattern extraction
- **Correctness**: Deterministic, reproducible pattern generation

## Workspace Structure
```
abstrie/
├── Cargo.toml                  # Workspace root
├── abstrie_core/               # Core framework crate
├── abstrie_templater/          # Log template strategy crate
├── abstrie_regexer/            # Regex synthesis strategy crate
├── bindings/
│   └── python/                 # Python bindings via PyO3
└── examples/                   # Usage examples and benchmarks
```

## Quick Start
Add the core crate to your workspace:
```toml
[dependencies]
abstrie_core = { path = "abstrie_core" }
```

Example usage:
```rust
use abstrie_core::GeneralizationTrie;

let mut trie = GeneralizationTrie::new();
for word in ["cat", "car", "dog", "dot"] {
    trie.insert(&word.chars().collect::<Vec<_>>());
}
println!("{}", trie.visualize_tree());
```

## Roadmap
- [x] Generic trie implementation
- [x] Insertion and traversal
- [x] Visualization utilities
- [ ] Node merging and pattern extraction
- [ ] Strategy trait implementations
- [ ] Python bindings and CLI
- [ ] Documentation and benchmarks

## Contributing
Contributions are welcome! See the requirements and design specification in `abstrie_requirements.md` for details on architecture, testing, and project phases.

## License
MIT
