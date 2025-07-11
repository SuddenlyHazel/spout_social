//! URI handling

use crate::common::id::Id;
use serde::{Deserialize, Serialize};
use std::fmt;

/// URI types supported by the system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UriType {
    Group,
    Topic,
    Post,
    Comment,
}

impl UriType {
    fn as_str(&self) -> &'static str {
        match self {
            UriType::Group => "group",
            UriType::Topic => "topic",
            UriType::Post => "post",
            UriType::Comment => "comment",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "group" => Some(UriType::Group),
            "topic" => Some(UriType::Topic),
            "post" => Some(UriType::Post),
            "comment" => Some(UriType::Comment),
            _ => None,
        }
    }
}

/// Parsed URI components
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedUri {
    pub uri_type: UriType,
    pub id: String,
    pub hints: Option<Vec<String>>,
}

/// URI helper for spout protocol
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Uri(String);

impl Uri {
    /// Create a URI from a raw string (for parsing/deserialization)
    pub fn from_string(uri: String) -> Self {
        Self(uri)
    }

    /// Create a group URI
    pub fn group<T: Id>(group_id: &T) -> Self {
        Self(format!("spout:group:{}", group_id.as_str()))
    }

    /// Create a topic URI with optional routing hints
    pub fn topic<T: Id>(topic_id: &T, hints: Option<&[String]>) -> Self {
        Self::build_uri(UriType::Topic, topic_id.as_str(), hints)
    }

    /// Create a post URI with optional routing hints
    pub fn post<T: Id>(post_id: &T, hints: Option<&[String]>) -> Self {
        Self::build_uri(UriType::Post, post_id.as_str(), hints)
    }

    /// Create a comment URI with optional routing hints
    pub fn comment<T: Id>(comment_id: &T, hints: Option<&[String]>) -> Self {
        Self::build_uri(UriType::Comment, comment_id.as_str(), hints)
    }

    /// Build URI with optional hints
    fn build_uri(uri_type: UriType, id: &str, hints: Option<&[String]>) -> Self {
        match hints {
            Some(hint_list) if !hint_list.is_empty() => {
                let hint_str = hint_list.join(".");
                Self(format!("spout:{}:{}@{}", uri_type.as_str(), id, hint_str))
            }
            _ => Self(format!("spout:{}:{}", uri_type.as_str(), id)),
        }
    }

    /// Build routing hints from hierarchical ID components
    pub fn build_hints<G: Id, T: Id, P: Id>(
        group_id: &G,
        topic_id: Option<&T>,
        post_id: Option<&P>,
        hint_bits: u8,
    ) -> Vec<String> {
        let mut hints = Vec::new();

        // Always include group hint
        if let Some(group_hint) = group_id.hint(hint_bits) {
            hints.push(group_hint);
        }

        // Add topic hint if present
        if let Some(topic) = topic_id {
            if let Some(topic_hint) = topic.hint(hint_bits) {
                hints.push(topic_hint);
            }
        }

        // Add post hint if present
        if let Some(post) = post_id {
            if let Some(post_hint) = post.hint(hint_bits) {
                hints.push(post_hint);
            }
        }

        hints
    }

    /// Parse a URI into its components
    pub fn parse(&self) -> Option<ParsedUri> {
        let parts: Vec<&str> = self.0.split(':').collect();
        if parts.len() != 3 || parts[0] != "spout" {
            return None;
        }

        let uri_type = UriType::from_str(parts[1])?;
        let id_and_hints = parts[2];

        if let Some(at_pos) = id_and_hints.find('@') {
            let (id, hints_str) = id_and_hints.split_at(at_pos);
            let hints: Vec<String> = hints_str[1..] // Skip the '@'
                .split('.')
                .map(|s| s.to_string())
                .collect();

            Some(ParsedUri {
                uri_type,
                id: id.to_string(),
                hints: Some(hints),
            })
        } else {
            Some(ParsedUri {
                uri_type,
                id: id_and_hints.to_string(),
                hints: None,
            })
        }
    }

    /// Get just the ID portion of the URI
    pub fn id(&self) -> Option<String> {
        self.parse().map(|p| p.id)
    }

    /// Get the URI type
    pub fn uri_type(&self) -> Option<UriType> {
        self.parse().map(|p| p.uri_type)
    }

    /// Get the routing hints if present
    pub fn hints(&self) -> Option<Vec<String>> {
        self.parse().and_then(|p| p.hints)
    }

    /// Check if URI has routing hints
    pub fn has_hints(&self) -> bool {
        self.0.contains('@')
    }

    /// Get the URI as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if this URI is valid
    pub fn is_valid(&self) -> bool {
        self.parse().is_some()
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

impl From<&str> for Uri {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uri_creation() {
        // Mock ID for testing
        #[derive(Debug, Clone)]
        struct TestId(String);
        impl Id for TestId {
            fn as_str(&self) -> &str {
                &self.0
            }
        }
        impl std::fmt::Display for TestId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        let id = TestId("abc123".to_string());
        let uri = Uri::post(&id, None);
        assert_eq!(uri.as_str(), "spout:post:abc123");

        let hints = vec!["hint1".to_string(), "hint2".to_string()];
        let uri_with_hints = Uri::post(&id, Some(&hints));
        assert_eq!(uri_with_hints.as_str(), "spout:post:abc123@hint1.hint2");
    }

    #[test]
    fn test_uri_parsing() {
        let uri = Uri::from_string("spout:post:abc123@hint1.hint2".to_string());
        let parsed = uri.parse().unwrap();

        assert_eq!(parsed.uri_type, UriType::Post);
        assert_eq!(parsed.id, "abc123");
        assert_eq!(
            parsed.hints,
            Some(vec!["hint1".to_string(), "hint2".to_string()])
        );
    }

    #[test]
    fn test_uri_without_hints() {
        let uri = Uri::from_string("spout:group:xyz789".to_string());
        let parsed = uri.parse().unwrap();

        assert_eq!(parsed.uri_type, UriType::Group);
        assert_eq!(parsed.id, "xyz789");
        assert_eq!(parsed.hints, None);
    }
}
