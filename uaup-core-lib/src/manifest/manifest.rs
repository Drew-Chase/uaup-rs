use crate::manifest_data::{Architecture, FileHash, Platform, PlatformItem, VersionItem, OS};
use sha2::Digest;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(feature = "create")]
pub fn create(version: impl AsRef<str>, platforms: Vec<Platform>, release_notes: impl AsRef<str>) -> Result<VersionItem, Box<dyn Error>> {
	let mut platform_items: Vec<PlatformItem> = vec![];
	for platform in platforms {
		let platform_item = generate_platform_item(platform.os, platform.architecture, platform.files)?;
		platform_items.push(platform_item);
	}

	Ok(VersionItem {
		version: version.as_ref().to_string(),
		platforms: platform_items,
		release_date: chrono::Utc::now().naive_utc(),
		release_notes: release_notes.as_ref().to_string()
	})
}


fn generate_update_archive(files: Vec<String>, platform_item: PlatformItem) -> Result<PathBuf, Box<dyn Error>>
{
	Ok(PathBuf::new())
}

fn generate_platform_item(os: OS, architecture: Architecture, files: Vec<String>) -> Result<PlatformItem, Box<dyn Error>>
{
	let mut size = 0;
	let mut file_hashes: Vec<FileHash> = vec![];
	for file in files {
		let hash = generate_hash(&file)?;
		let metadata = fs::metadata(&file)?;
		let file_size = metadata.len();
		file_hashes.push(FileHash {
			file: file.clone(),
			hash,
			size: file_size
		});
		size += file_size;
	}


	Ok(PlatformItem {
		os,
		architecture,
		sha256: file_hashes,
		size
	})
}


fn generate_hash(file: impl AsRef<Path>) -> Result<String, Box<dyn Error>>
{
	let mut stream = fs::File::open(file.as_ref())?;
	let mut hasher = sha2::Sha256::new();
	std::io::copy(&mut stream, &mut hasher)?;
	let hash = hasher.finalize();
	let hash = base16ct::lower::encode_string(&hash);
	Ok(hash)
}