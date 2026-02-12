//! Core implementation of the Quantum-Resistant Verkle Tree commitment scheme
//! 
//! This module contains the implementation of:
//! - Quantum-Resistant Verkle Tree data structure
//! - Commitment schemes
//! - Proof generation and verification
//! - Error handling
//! - Optimized multiproof implementation (std-only)

pub mod schemes;
pub mod scheme_tree;
pub mod errors;
#[cfg(feature = "std")]
pub mod multiproof;

// Re-exports
pub use schemes::{
    CommitmentScheme,
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
};
pub use scheme_tree::{QuantumTree, QuantumProof, QuantumRangeProof, QuantumMultiProof};
pub use errors::VerkleError;
#[cfg(feature = "std")]
pub use multiproof::sha3::Sha3_256QuantumTree;
