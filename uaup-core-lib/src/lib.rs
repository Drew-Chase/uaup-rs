#[cfg(feature = "github")]
#[path = "github/github.rs"]
mod github;
#[cfg(feature = "manifest")]
#[path = "manifest/manifest.rs"]
mod manifest;

#[cfg(feature = "manifest")]
#[path = "manifest/manifest_data.rs"]
mod manifest_data;
#[cfg(feature = "update_server")]
#[path = "manifest/update_server.rs"]
mod update_server;
