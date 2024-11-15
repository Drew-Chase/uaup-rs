#[cfg(feature = "github")]
#[path = "github/github.rs"]
pub mod github;
#[cfg(feature = "manifest")]
#[path = "manifest/manifest.rs"]
pub mod manifest;

#[cfg(feature = "manifest")]
#[path = "manifest/manifest_data.rs"]
pub mod manifest_data;
#[cfg(feature = "update_server")]
#[path = "manifest/update_server.rs"]
pub mod update_server;
