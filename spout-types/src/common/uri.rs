//! URI handling for custom scheme

use serde::{Deserialize, Serialize};
use std::fmt;

/// Custom URI type for our protocol
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Uri(String);

impl Uri {
    /// Create a new URI from string
    pub fn new(uri: String) -> Self {
        Self(uri)
    }

    /// Create a topic URI with hint
    pub fn topic(group_hint: &str, topic_id: &str) -> Self {
        Self(format!("spout:topic:{}.{}", group_hint, topic_id))
    }

    /// Create a post URI with hint
    pub fn post(group_hint: &str, post_id: &str) -> Self {
        Self(format!("spout:post:{}.{}", group_hint, post_id))
    }

    /// Create a reply URI with hint
    pub fn reply(group_hint: &str, reply_id: &str) -> Self {
        Self(format!("spout:reply:{}.{}", group_hint, reply_id))
    }

    /// Get the URI as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Uri {
    fn from(s: String) -> Self {
        Self(s)
    }
}
