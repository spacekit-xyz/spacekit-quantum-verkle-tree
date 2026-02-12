#![cfg_attr(not(feature = "std"), no_std)]
//! Quantum-Resistant Verkle Tree Implementation
//!
//! This library provides a Quantum-Resistant Verkle Tree implementation, which is an efficient
//! data structure for storing and verifying key-value pairs.

extern crate alloc;

pub mod commitment;

// Re-export commonly used types
pub use commitment::{
    errors::VerkleError,
    scheme_tree::{QuantumTree, QuantumProof, QuantumRangeProof, QuantumMultiProof},
    schemes::{
        HashCommitmentScheme,
        LatticeCommitmentScheme,
        LatticeOpening,
        LatticeParameterSet,
        Kyber512Params,
        Kyber768Params,
        Kyber1024Params,
        WeeWuSisParams,
        Sis128B,
        Sis128HB,
        Sis192B,
        Sis192HB,
        SisOpening,
        WeeWuSisCommitmentScheme,
        NistSisScheme,
        SisSecurityLevel,
        SisProfile,
        setup_sis_params,
    },
};
// Optional: provide convenience functions at the root level
pub fn new_quantum_tree() -> QuantumTree<NistSisScheme> {
    QuantumTree::new()
}
