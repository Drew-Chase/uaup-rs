use crate::manifest_data::{Architecture, FileHash, Platform, PlatformItem, VersionItem, OS};
use log::debug;
use lzma_tarball::writer::{ArchiveEntry, LZMATarballWriter};
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

	for platform in &platform_items {
		generate_update_archive(platform, version.as_ref())?;
	}

	Ok(VersionItem {
		version: version.as_ref().to_string(),
		platforms: platform_items,
		release_date: chrono::Utc::now().naive_utc(),
		release_notes: release_notes.as_ref().to_string()
	})
}


fn generate_update_archive(platform_item: impl AsRef<PlatformItem>, version: impl AsRef<str>) -> Result<PathBuf, Box<dyn Error>>
{
	let platform_item = platform_item.as_ref();
	let files: Vec<String> = platform_item.sha256.iter().map(|f| f.file.clone()).collect();
	let root = calculate_root(&files);
	let mut files: Vec<ArchiveEntry> = files.iter().map(|f|
		ArchiveEntry {
			archive_path: f.strip_prefix(&root).unwrap_or(f).to_string(),
			filesystem_path: PathBuf::from(f)
		}
	).collect();

	let output_file = PathBuf::from(format!("{}-{}-{}.tar.xz", platform_item.os, platform_item.architecture, version.as_ref()));
	LZMATarballWriter::new()
		.with_files(&mut files)
		.set_compression_level(0)
		.set_output(&output_file)
		.compress(|progress| {
			debug!("Compression: {:?}", progress);
		})?;
	Ok(output_file)
}

pub fn calculate_root(file_paths: &Vec<String>) -> String
{
	let mut root = PathBuf::from(&file_paths[0]);
	for file in file_paths {
		let file_path = Path::new(file);
		while !file_path.starts_with(&root) {
			root.pop();
		}
	}
	root.to_string_lossy().into_owned()
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