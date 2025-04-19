pub mod monad;

use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub type Result<T, E = BlockchainError> = std::result::Result<T, E>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockchainError {
    CryptoError(String),
    ValidationError(String),
    StateError(String),
    StorageError(String),
    NetworkError(String),
    Other(String),
}

impl Display for BlockchainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockchainError::CryptoError(msg) => write!(f, "Crypto Error: {}", msg),
            BlockchainError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            BlockchainError::StateError(msg) => write!(f, "State Error: {}", msg),
            BlockchainError::StorageError(msg) => write!(f, "Storage Error: {}", msg),
            BlockchainError::NetworkError(msg) => write!(f, "Network Error: {}", msg),
            BlockchainError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for BlockchainError {}
