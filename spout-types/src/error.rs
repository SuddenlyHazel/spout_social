//! Error types for the API

use thiserror::Error;

/// Common result type used throughout the API
pub type Result<T> = std::result::Result<T, ApiError>;

/// Main error type for API operations
#[derive(Error, Debug)]
pub enum ApiError {
    // ID and URI errors
    #[error("Invalid ID format: {0}")]
    InvalidId(String),

    #[error("Invalid URI format: {0}")]
    InvalidUri(String),

    #[error("Invalid ID hint: {hint_bits} bits for ID {id}")]
    InvalidIdHint { id: String, hint_bits: u8 },

    // Cryptographic errors
    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),

    #[error("Signature verification failed")]
    SignatureVerificationFailed,

    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),

    // Content errors
    #[error("Content too large: {size} bytes (max: {max})")]
    ContentTooLarge { size: usize, max: usize },

    #[error("Invalid content type: {0}")]
    InvalidContentType(String),

    #[error("Missing required field: {0}")]
    MissingRequiredField(String),

    // Community errors
    #[error("Access denied: insufficient permissions")]
    AccessDenied,

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Community not found: {0}")]
    CommunityNotFound(String),

    #[error("Already a member of community: {0}")]
    AlreadyMember(String),

    // Time errors
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),

    #[error("Timestamp too old: {0}")]
    TimestampTooOld(String),

    #[error("Timestamp in future: {0}")]
    TimestampInFuture(String),

    // Serialization errors
    #[error("Base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),

    #[error("UUID parse error: {0}")]
    UuidParse(#[from] uuid::Error),

    // Protocol errors
    #[error("Protocol version mismatch: expected {expected}, got {actual}")]
    ProtocolVersionMismatch { expected: u32, actual: u32 },

    #[error("Malformed message: {0}")]
    MalformedMessage(String),

    #[error("Unknown message type: {0}")]
    UnknownMessageType(String),

    // Generic errors
    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl ApiError {
    /// Create a new cryptographic error
    pub fn crypto(msg: impl Into<String>) -> Self {
        Self::Internal(format!("Crypto error: {}", msg.into()))
    }

    /// Create a new validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::ValidationFailed(msg.into())
    }

    /// Create a new internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}

// Convert from ed25519 signature errors
impl From<ed25519_dalek::SignatureError> for ApiError {
    fn from(err: ed25519_dalek::SignatureError) -> Self {
        Self::crypto(format!("Ed25519 signature error: {}", err))
    }
}

// Convert from chrono parsing errors
impl From<chrono::ParseError> for ApiError {
    fn from(err: chrono::ParseError) -> Self {
        Self::InvalidTimestamp(err.to_string())
    }
}
