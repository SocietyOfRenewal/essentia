use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChainError {
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("already exists: {0}")]
    AlreadyExists(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("invalid did: {0}")]
    InvalidDid(String),
    #[error("invalid signature")]
    InvalidSignature,
    #[error("invalid transaction: {0}")]
    InvalidTransaction(String),
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("nonce mismatch for {signer}: expected {expected}, got {got}")]
    NonceMismatch {
        signer: String,
        expected: u64,
        got: u64,
    },
    #[error("insufficient balance for {owner} in {asset}: available {available}, required {required}")]
    InsufficientBalance {
        owner: String,
        asset: String,
        available: u64,
        required: u64,
    },
    #[error("io error: {0}")]
    Io(String),
    #[error("serialization error: {0}")]
    Serialization(String),
}

impl From<std::io::Error> for ChainError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

impl From<serde_json::Error> for ChainError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialization(value.to_string())
    }
}
