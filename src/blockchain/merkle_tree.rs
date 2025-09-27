//! Merkle tree implementation
//!
//! This module provides Merkle tree functionality for Unicoin blockchain.

use crate::crypto::Hash;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Merkle tree proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    /// The leaf hash being proven
    pub leaf_hash: Hash,
    /// Path of hashes from leaf to root
    pub path: Vec<MerklePathNode>,
    /// Index of the leaf in the tree
    pub leaf_index: usize,
}

/// Node in the Merkle proof path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerklePathNode {
    /// Hash of the sibling node
    pub sibling_hash: Hash,
    /// Whether this sibling is on the left (true) or right (false)
    pub is_left: bool,
}

/// Merkle tree structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleTree {
    /// All nodes in the tree (indexed by level and position)
    nodes: Vec<Vec<Hash>>,
    /// Number of leaf nodes
    leaf_count: usize,
    /// Root hash of the tree
    root_hash: Hash,
}

impl MerkleTree {
    /// Create a new Merkle tree from a list of hashes
    pub fn new(leaf_hashes: Vec<Hash>) -> Self {
        if leaf_hashes.is_empty() {
            return Self {
                nodes: vec![vec![Hash::zero()]],
                leaf_count: 0,
                root_hash: Hash::zero(),
            };
        }

        let leaf_count = leaf_hashes.len();
        let mut nodes = Vec::new();
        
        // Pad leaves to power of 2
        let mut current_level = leaf_hashes;
        while current_level.len() % 2 != 0 {
            current_level.push(current_level.last().unwrap().clone());
        }
        
        nodes.push(current_level.clone());

        // Build tree bottom-up
        let mut level_index = 0;
        while nodes[level_index].len() > 1 {
            let current_level = &nodes[level_index];
            let mut next_level = Vec::new();

            for i in (0..current_level.len()).step_by(2) {
                let left = current_level[i];
                let right = current_level[i + 1];
                let parent = Self::combine_hashes(&left, &right);
                next_level.push(parent);
            }

            nodes.push(next_level);
            level_index += 1;
        }

        let root_hash = nodes.last().unwrap()[0];
        
        Self {
            nodes,
            leaf_count,
            root_hash,
        }
    }

    /// Get the root hash of the tree
    pub fn root_hash(&self) -> Hash {
        self.root_hash
    }

    /// Get the number of leaf nodes
    pub fn leaf_count(&self) -> usize {
        self.leaf_count
    }

    /// Generate a Merkle proof for a leaf at the given index
    pub fn generate_proof(&self, leaf_index: usize) -> Option<MerkleProof> {
        if leaf_index >= self.leaf_count {
            return None;
        }

        let leaf_hash = self.nodes[0][leaf_index];
        let mut path = Vec::new();
        let mut current_index = leaf_index;

        // Build proof path from leaf to root
        for level in 0..self.nodes.len() - 1 {
            let level_nodes = &self.nodes[level];
            
            // Determine sibling position
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            // Add sibling to proof path
            if sibling_index < level_nodes.len() {
                path.push(MerklePathNode {
                    sibling_hash: level_nodes[sibling_index],
                    is_left: sibling_index < current_index,
                });
            }

            // Move to parent level
            current_index /= 2;
        }

        Some(MerkleProof {
            leaf_hash,
            path,
            leaf_index,
        })
    }

    /// Verify a Merkle proof
    pub fn verify_proof(proof: &MerkleProof, root_hash: &Hash) -> bool {
        let mut current_hash = proof.leaf_hash;

        // Reconstruct the path from leaf to root
        for path_node in &proof.path {
            if path_node.is_left {
                // Sibling is on the left, current is on the right
                current_hash = Self::combine_hashes(&path_node.sibling_hash, &current_hash);
            } else {
                // Current is on the left, sibling is on the right
                current_hash = Self::combine_hashes(&current_hash, &path_node.sibling_hash);
            }
        }

        current_hash == *root_hash
    }

    /// Combine two hashes to create a parent hash
    fn combine_hashes(left: &Hash, right: &Hash) -> Hash {
        let mut combined = Vec::new();
        combined.extend_from_slice(left.as_bytes());
        combined.extend_from_slice(right.as_bytes());
        crate::crypto::double_sha256(&combined)
    }

    /// Get all leaf hashes
    pub fn get_leaf_hashes(&self) -> Vec<Hash> {
        if self.nodes.is_empty() {
            return Vec::new();
        }
        
        self.nodes[0][..self.leaf_count].to_vec()
    }

    /// Update a leaf hash and recalculate the tree
    pub fn update_leaf(&mut self, leaf_index: usize, new_hash: Hash) -> bool {
        if leaf_index >= self.leaf_count {
            return false;
        }

        // Update the leaf
        self.nodes[0][leaf_index] = new_hash;

        // Recalculate the tree
        let mut level = 0;
        while level < self.nodes.len() - 1 {
            let current_level = &self.nodes[level];
            let mut next_level = Vec::new();

            for i in (0..current_level.len()).step_by(2) {
                let left = current_level[i];
                let right = if i + 1 < current_level.len() {
                    current_level[i + 1]
                } else {
                    left // Duplicate last node if odd number
                };
                let parent = Self::combine_hashes(&left, &right);
                next_level.push(parent);
            }

            self.nodes[level + 1] = next_level;
            level += 1;
        }

        // Update root hash
        self.root_hash = self.nodes.last().unwrap()[0];
        true
    }

    /// Get tree statistics
    pub fn get_statistics(&self) -> MerkleTreeStatistics {
        let height = if self.nodes.is_empty() { 0 } else { self.nodes.len() };
        let total_nodes: usize = self.nodes.iter().map(|level| level.len()).sum();

        MerkleTreeStatistics {
            height,
            leaf_count: self.leaf_count,
            total_nodes,
            root_hash: self.root_hash,
        }
    }
}

/// Merkle tree statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleTreeStatistics {
    pub height: usize,
    pub leaf_count: usize,
    pub total_nodes: usize,
    pub root_hash: Hash,
}

/// Sparse Merkle tree for efficient updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparseMerkleTree {
    /// Tree height (number of levels)
    height: usize,
    /// Default hash for empty subtrees
    default_hashes: Vec<Hash>,
    /// Actual nodes in the tree
    nodes: HashMap<u64, Hash>,
    /// Root hash
    root_hash: Hash,
}

impl SparseMerkleTree {
    /// Create a new sparse Merkle tree
    pub fn new(height: usize) -> Self {
        let mut default_hashes = Vec::new();
        
        // Precompute default hashes for each level
        let mut current_default = Hash::zero();
        default_hashes.push(current_default);
        
        for _ in 1..height {
            current_default = Self::combine_hashes(&current_default, &current_default);
            default_hashes.push(current_default);
        }

        Self {
            height,
            default_hashes,
            nodes: HashMap::new(),
            root_hash: default_hashes[height - 1],
        }
    }

    /// Set a value at the given key
    pub fn set(&mut self, key: u64, value: Hash) {
        let mut current_key = key;
        let mut current_hash = value;

        // Update the leaf
        self.nodes.insert(current_key, current_hash);

        // Update path to root
        for level in 0..self.height - 1 {
            let sibling_key = current_key ^ 1;
            let sibling_hash = self.nodes.get(&sibling_key)
                .copied()
                .unwrap_or(self.default_hashes[level]);

            let parent_key = current_key >> 1;
            let parent_hash = if current_key % 2 == 0 {
                Self::combine_hashes(&current_hash, &sibling_hash)
            } else {
                Self::combine_hashes(&sibling_hash, &current_hash)
            };

            self.nodes.insert(parent_key, parent_hash);
            current_key = parent_key;
            current_hash = parent_hash;
        }

        self.root_hash = current_hash;
    }

    /// Get a value at the given key
    pub fn get(&self, key: u64) -> Option<Hash> {
        self.nodes.get(&key).copied()
    }

    /// Generate a proof for a key
    pub fn generate_proof(&self, key: u64) -> MerkleProof {
        let leaf_hash = self.nodes.get(&key).copied().unwrap_or(Hash::zero());
        let mut path = Vec::new();
        let mut current_key = key;

        for level in 0..self.height - 1 {
            let sibling_key = current_key ^ 1;
            let sibling_hash = self.nodes.get(&sibling_key)
                .copied()
                .unwrap_or(self.default_hashes[level]);

            path.push(MerklePathNode {
                sibling_hash,
                is_left: sibling_key < current_key,
            });

            current_key >>= 1;
        }

        MerkleProof {
            leaf_hash,
            path,
            leaf_index: key as usize,
        }
    }

    /// Get the root hash
    pub fn root_hash(&self) -> Hash {
        self.root_hash
    }

    /// Combine two hashes
    fn combine_hashes(left: &Hash, right: &Hash) -> Hash {
        let mut combined = Vec::new();
        combined.extend_from_slice(left.as_bytes());
        combined.extend_from_slice(right.as_bytes());
        crate::crypto::double_sha256(&combined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree_creation() {
        let hashes = vec![
            Hash::from_string("leaf1"),
            Hash::from_string("leaf2"),
            Hash::from_string("leaf3"),
            Hash::from_string("leaf4"),
        ];

        let tree = MerkleTree::new(hashes);
        assert_eq!(tree.leaf_count(), 4);
        assert!(!tree.root_hash().is_zero());
    }

    #[test]
    fn test_merkle_proof_generation() {
        let hashes = vec![
            Hash::from_string("leaf1"),
            Hash::from_string("leaf2"),
            Hash::from_string("leaf3"),
            Hash::from_string("leaf4"),
        ];

        let tree = MerkleTree::new(hashes);
        let proof = tree.generate_proof(0).unwrap();

        assert_eq!(proof.leaf_hash, Hash::from_string("leaf1"));
        assert_eq!(proof.leaf_index, 0);
        assert!(!proof.path.is_empty());
    }

    #[test]
    fn test_merkle_proof_verification() {
        let hashes = vec![
            Hash::from_string("leaf1"),
            Hash::from_string("leaf2"),
            Hash::from_string("leaf3"),
            Hash::from_string("leaf4"),
        ];

        let tree = MerkleTree::new(hashes);
        let proof = tree.generate_proof(0).unwrap();
        let root_hash = tree.root_hash();

        assert!(MerkleTree::verify_proof(&proof, &root_hash));
    }

    #[test]
    fn test_sparse_merkle_tree() {
        let mut sparse_tree = SparseMerkleTree::new(4);
        
        sparse_tree.set(0, Hash::from_string("value0"));
        sparse_tree.set(1, Hash::from_string("value1"));
        
        assert_eq!(sparse_tree.get(0), Some(Hash::from_string("value0")));
        assert_eq!(sparse_tree.get(1), Some(Hash::from_string("value1")));
        assert_eq!(sparse_tree.get(2), None);

        let proof = sparse_tree.generate_proof(0);
        assert!(MerkleTree::verify_proof(&proof, &sparse_tree.root_hash()));
    }
}
