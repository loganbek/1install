//! SHA-256 verification logic

use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Hash mismatch: expected {expected}, but found {actual}")]
    HashMismatch {
        expected: String,
        actual: String,
    },
}

/// Verify that a file's SHA-256 hash matches the expected hash
pub fn verify_file_hash(path: &Path, expected_hash: &str) -> Result<(), VerificationError> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    
    let result = hasher.finalize();
    let actual_hash = hex::encode(result);
    
    if actual_hash.to_lowercase() == expected_hash.to_lowercase() {
        Ok(())
    } else {
        Err(VerificationError::HashMismatch {
            expected: expected_hash.to_string(),
            actual: actual_hash,
        })
    }
}
