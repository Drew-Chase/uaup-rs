use serde::{Deserialize, Serialize};
use std::fmt::Display;
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

impl AsRef<PlatformItem> for PlatformItem {
	fn as_ref(&self) -> &PlatformItem {
		self
	}
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

impl Display for OS {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let str = match self {
			OS::Windows => "windows".to_string(),
			OS::Linux => "linux".to_string(),
			OS::MacOS => "macos".to_string()
		};
		write!(f, "{}", str)
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

impl Display for Architecture {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let str = match self {
			Architecture::X86 => "x86".to_string(),
			Architecture::X86_64 => "x86_64".to_string(),
			Architecture::ARM => "arm".to_string(),
			Architecture::ARM64 => "arm64".to_string()
		};
		write!(f, "{}", str)
	}
}