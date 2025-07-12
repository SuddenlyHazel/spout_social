//! API Types for P2P Social Media
//!
//! This crate contains all the shared types used across the application,
//! including identity, content, community, and activity types.
//!
//! # Basic Usage
//!
//! ```rust
//! use spout_types::{Handle, Profile, base64url_id};
//!
//! let profile = Profile::new("Alice".to_string());
//! let handle = Handle::new(profile);
//! let id = base64url_id();
//! ```

// Re-export commonly used types for convenience
pub use actor::{Handle, HandleError, HandleId};
pub use common::{Id, Timestamp, Uri, base64url_id, id_hint};
pub use content::Profile;
//pub use community;

// Module declarations
pub mod actor;
pub mod common;
pub mod community;
pub mod content;

// Error handling
pub mod error;
pub use error::{ApiError, Result};
