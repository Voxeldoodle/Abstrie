//! Abstrie: A trie-based pattern abstraction framework
//! 
//! This crate provides functionality for building and manipulating tries
//! that can be used for pattern recognition and abstraction.

pub mod trie;
pub mod visualization;

// Re-export all public items from trie module
pub use trie::{
    TrieNode,
    LengthGroupedNode,
};

// Provide a prelude for convenient imports
pub mod prelude {
    pub use crate::{
        TrieNode,
        LengthGroupedNode,
    };
}
