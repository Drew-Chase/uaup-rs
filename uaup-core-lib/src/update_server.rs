use serde::{Deserialize, Serialize};

pub struct UpdateServer {
	host: String,
	port: u16,
	secure: bool,
	base_path: String
}

impl<M> UpdateServer
where
	M: Serialize + for<'de> Deserialize<'de>
{
	pub fn new(host: String, port: u16, secure: bool, base_path: Option<String>) -> Self {
		UpdateServer {
			host,
			port,
			base_path: base_path.unwrap_or("".to_string()),
			secure
		}
	}
	pub async fn get_updates(&self) -> Result<Vec<M>, String> {
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
			.json::<Vec<M>>()
			.await
			.map_err(|e| e.to_string())
	}
}