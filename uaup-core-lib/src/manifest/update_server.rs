use serde::{Deserialize, Serialize};
use crate::manifest_data::VersionItem;

pub struct UpdateServer {
	host: String,
	port: u16,
	secure: bool,
	base_path: String
}

impl UpdateServer
{
	pub fn new(host: String, port: u16, secure: bool, base_path: Option<String>) -> Self {
		UpdateServer {
			host,
			port,
			base_path: base_path.unwrap_or("".to_string()),
			secure
		}
	}
	pub async fn get_updates(&self) -> Result<Vec<VersionItem>, String> {
		reqwest::get(
			&format!(
				"{protocol}://{host}:{port}/{base_path}/updates",
				protocol = if self.secure { "https" } else { "http" },
				host = self.host,
				port = self.port,
				base_path = self.base_path
			)
		)
			.await
			.map_err(|e| e.to_string())?
			.json::<Vec<VersionItem>>()
			.await
			.map_err(|e| e.to_string())
	}
}