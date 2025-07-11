//! API Types for P2P Social Media
//!
//! This crate contains all the shared types used across the application,
//! including identity, content, community, and activity types.
//!
//! # Basic Usage
//!
//! ```rust
//! use api_types::{DeviceId, AccountId, Post, Community};
//!
//! let device_id = DeviceId::new();
//! let account_id = AccountId::new();
//! ```

// Re-export commonly used types for convenience
pub use common::{Id, Timestamp, Uri, base64url_id, id_hint};
//pub use community;
//pub use content;

// Module declarations
pub mod common;
pub mod community;
pub mod content;

// Error handling
pub mod error;
pub use error::{ApiError, Result};
