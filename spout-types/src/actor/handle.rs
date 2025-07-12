//! Handle type for actor identity with keypair and profile

use crate::common::Id;
use crate::content::Profile;
use base64::prelude::*;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Handle ID derived from public key
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HandleId(String);

impl HandleId {
    /// Create a HandleId from a public key
    pub fn from_public_key(public_key: &VerifyingKey) -> Self {
        let encoded = BASE64_URL_SAFE_NO_PAD.encode(public_key.as_bytes());
        Self(encoded)
    }

    /// Create a HandleId from a string (for deserialization)
    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    /// Get the public key bytes if valid
    pub fn to_public_key(&self) -> Option<VerifyingKey> {
        let bytes = BASE64_URL_SAFE_NO_PAD.decode(&self.0).ok()?;
        let bytes_array: [u8; 32] = bytes.try_into().ok()?;
        VerifyingKey::from_bytes(&bytes_array).ok()
    }
}

impl Id for HandleId {
    fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HandleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for HandleId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Actor handle with identity and profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handle {
    /// Unique identifier derived from public key
    pub id: HandleId,
    /// Ed25519 keypair for signing
    #[serde(skip)]
    pub keypair: Option<SigningKey>,
    /// Public key (always available for verification)
    pub public_key: VerifyingKey,
    /// Profile information
    pub profile: Profile,
}

impl Handle {
    /// Create a new handle with a generated keypair
    pub fn new(profile: Profile) -> Self {
        let secret_bytes = rand::random::<[u8; 32]>();
        let keypair = SigningKey::from_bytes(&secret_bytes);
        let public_key = keypair.verifying_key();
        let id = HandleId::from_public_key(&public_key);

        Self {
            id,
            keypair: Some(keypair),
            public_key,
            profile,
        }
    }

    /// Create a handle from an existing keypair
    pub fn from_keypair(keypair: SigningKey, profile: Profile) -> Self {
        let public_key = keypair.verifying_key();
        let id = HandleId::from_public_key(&public_key);

        Self {
            id,
            keypair: Some(keypair),
            public_key,
            profile,
        }
    }

    /// Create a handle for verification only (no private key)
    pub fn from_public_key(public_key: VerifyingKey, profile: Profile) -> Self {
        let id = HandleId::from_public_key(&public_key);

        Self {
            id,
            keypair: None,
            public_key,
            profile,
        }
    }

    /// Get the handle ID
    pub fn id(&self) -> &HandleId {
        &self.id
    }

    /// Get the profile
    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    /// Update the profile
    pub fn set_profile(&mut self, profile: Profile) {
        self.profile = profile;
    }

    /// Sign data with the private key
    pub fn sign(&self, data: &[u8]) -> Result<Signature, HandleError> {
        let keypair = self.keypair.as_ref().ok_or(HandleError::NoPrivateKey)?;
        Ok(keypair.sign(data))
    }

    /// Verify a signature against this handle's public key
    pub fn verify(&self, data: &[u8], signature: &Signature) -> Result<(), HandleError> {
        self.public_key
            .verify(data, signature)
            .map_err(|_| HandleError::InvalidSignature)
    }

    /// Check if this handle can sign (has private key)
    pub fn can_sign(&self) -> bool {
        self.keypair.is_some()
    }

    /// Get the public key
    pub fn public_key(&self) -> &VerifyingKey {
        &self.public_key
    }

    /// Export the keypair for storage (private key included)
    pub fn export_keypair(&self) -> Option<Vec<u8>> {
        self.keypair.as_ref().map(|kp| kp.to_bytes().to_vec())
    }

    /// Import a keypair from bytes
    pub fn import_keypair(&mut self, bytes: &[u8]) -> Result<(), HandleError> {
        let bytes_array: [u8; 32] = bytes.try_into().map_err(|_| HandleError::InvalidKeypair)?;
        let keypair = SigningKey::from_bytes(&bytes_array);

        // Verify the public key matches
        if keypair.verifying_key() != self.public_key {
            return Err(HandleError::KeypairMismatch);
        }

        self.keypair = Some(keypair);
        Ok(())
    }
}

impl fmt::Display for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.profile.name(), self.id)
    }
}

impl PartialEq for Handle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.public_key == other.public_key
    }
}

impl Eq for Handle {}

/// Errors that can occur when working with handles
#[derive(Debug, thiserror::Error)]
pub enum HandleError {
    #[error("No private key available for signing")]
    NoPrivateKey,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Invalid keypair format")]
    InvalidKeypair,
    #[error("Keypair does not match public key")]
    KeypairMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_creation() {
        let profile = Profile::new("Test User".to_string());
        let handle = Handle::new(profile);

        assert_eq!(handle.profile().name(), "Test User");
        assert!(handle.can_sign());
    }

    #[test]
    fn test_handle_signing() {
        let profile = Profile::new("Signer".to_string());
        let handle = Handle::new(profile);

        let data = b"hello world";
        let signature = handle.sign(data).unwrap();

        assert!(handle.verify(data, &signature).is_ok());
    }

    #[test]
    fn test_handle_verification_only() {
        let profile = Profile::new("Verifier".to_string());
        let full_handle = Handle::new(profile.clone());

        let verify_handle = Handle::from_public_key(full_handle.public_key.clone(), profile);

        assert!(!verify_handle.can_sign());
        assert_eq!(verify_handle.id(), full_handle.id());
    }

    #[test]
    fn test_handle_id_from_public_key() {
        let profile = Profile::new("Test".to_string());
        let handle = Handle::new(profile);

        let recovered_pk = handle.id().to_public_key().unwrap();
        assert_eq!(recovered_pk, handle.public_key);
    }
}
