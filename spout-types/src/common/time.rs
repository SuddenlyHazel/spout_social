//! Timestamp utilities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Standard timestamp type used throughout the API
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    /// Create a new timestamp with the current time
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Create from a DateTime<Utc>
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    /// Get the inner DateTime
    pub fn datetime(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}
