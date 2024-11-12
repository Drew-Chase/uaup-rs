use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub enum OS {
	Windows,
	Linux,
	MacOS
}

#[derive(Serialize, Deserialize)]
pub enum Architecture {
	X86,
	X86_64,
	ARM,
	ARM64
}

#[derive(Serialize, Deserialize)]
pub struct VersionItem {
	pub version: String,
	pub platforms: Vec<PlatformItem>,
	pub release_date: chrono::NaiveDateTime,
	pub release_notes: String
}

#[derive(Serialize, Deserialize)]
pub struct PlatformItem {
	pub os: OS,
	pub architecture: Architecture,
	pub sha256: Vec<FileHash>,
	pub size: u64,
}
pub struct Platform {
	pub os: OS,
	pub architecture: Architecture,
	pub files: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct FileHash {
	pub file: String,
	pub hash: String,
	pub size: u64
}

impl FromStr for OS {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"windows" => Ok(OS::Windows),
			"linux" => Ok(OS::Linux),
			"macos" => Ok(OS::MacOS),
			_ => Err("Invalid OS".to_string())
		}
	}
}

impl FromStr for Architecture {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"x86" => Ok(Architecture::X86),
			"x86_64" => Ok(Architecture::X86_64),
			"arm" => Ok(Architecture::ARM),
			"arm64" => Ok(Architecture::ARM64),
			_ => Err("Invalid Architecture".to_string())
		}
	}
}