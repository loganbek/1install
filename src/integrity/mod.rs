//! Integrity verification module

mod verifier;

pub use verifier::{verify_file_hash, VerificationError};
