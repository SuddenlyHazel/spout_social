pub mod protocol;
#[cfg(feature = "headless")]
pub mod tui;

pub const OCEAN_ALPN: &'static str = "social.spout.app.ocean.protocol.v0.1.0";
pub const OCEAN_APP_DB_KEY: &'static str = "social.spout.app.ocean.protocol.v0.1.0";

pub const OCEAN_PROFILE_KEY_BASE: &'static str = "social.spout.app.ocean.profile";
pub const OCEAN_POSTS_KEY_BASE: &'static str = "social.spout.app.ocean.posts";

pub const DOWNLOADED_PROFILES_TREE: &'static str = "social.spout.app.ocean.profiles.synced";
pub const BOOTSTRAP_NODE_PUBKEY: &'static str =
    "b72dbf3e68b0537b37f3ef58bb2b932095172f2537f525294e2c706ccd18f2f8";
