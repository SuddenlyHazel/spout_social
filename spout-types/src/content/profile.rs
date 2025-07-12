//! Profile content type for handles

use serde::{Deserialize, Serialize};
use std::fmt;

/// Profile information for a handle
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    /// Display name for the handle
    pub name: String,
    /// Optional profile image URL or data
    pub image: Option<String>,
}

impl Profile {
    /// Create a new profile with just a name
    pub fn new(name: String) -> Self {
        Self { name, image: None }
    }

    /// Create a new profile with name and image
    pub fn with_image(name: String, image: String) -> Self {
        Self {
            name,
            image: Some(image),
        }
    }

    /// Get the display name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the profile image if present
    pub fn image(&self) -> Option<&str> {
        self.image.as_deref()
    }

    /// Set the profile image
    pub fn set_image(&mut self, image: Option<String>) {
        self.image = image;
    }

    /// Update the display name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            name: "Anonymous".to_string(),
            image: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_creation() {
        let profile = Profile::new("Alice".to_string());
        assert_eq!(profile.name(), "Alice");
        assert_eq!(profile.image(), None);
    }

    #[test]
    fn test_profile_with_image() {
        let profile = Profile::with_image("Bob".to_string(), "avatar.jpg".to_string());
        assert_eq!(profile.name(), "Bob");
        assert_eq!(profile.image(), Some("avatar.jpg"));
    }

    #[test]
    fn test_profile_display() {
        let profile = Profile::new("Charlie".to_string());
        assert_eq!(format!("{}", profile), "Charlie");
    }
}
