//! ID types and utilities for base64url encoding

use base64::prelude::*;
use std::fmt;
use uuid::Uuid;

/// Generate a base64url-encoded UUID
pub fn base64url_id() -> String {
    let uuid = Uuid::new_v4();
    BASE64_URL_SAFE_NO_PAD.encode(uuid.as_bytes())
}

/// Extract hint from an ID (lower N bits)
pub fn id_hint(id: &str, hint_bits: u8) -> Option<String> {
    let bytes = BASE64_URL_SAFE_NO_PAD.decode(id).ok()?;
    if bytes.len() < 16 {
        return None;
    }

    // Take the hint bits from the last bytes
    let hint_bytes = hint_bits / 8;
    let start = bytes.len().saturating_sub(hint_bytes as usize);
    let hint = &bytes[start..];

    Some(BASE64_URL_SAFE_NO_PAD.encode(hint))
}

/// Base trait for all ID types
pub trait Id: Clone + fmt::Display + fmt::Debug + Send + Sync {
    fn as_str(&self) -> &str;
    fn hint(&self, bits: u8) -> Option<String> {
        id_hint(self.as_str(), bits)
    }
}

/// Macro to generate ID types
#[macro_export]
macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(String);

        impl $name {
            pub fn new() -> Self {
                Self(base64url_id())
            }

            pub fn from_string(s: String) -> Self {
                Self(s)
            }
        }

        impl Id for $name {
            fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }
    };
}
