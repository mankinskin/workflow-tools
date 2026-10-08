pub mod gates;
pub mod inventory;
pub mod model;

use sha2::{Digest, Sha256};
use std::path::{Component, Path, PathBuf};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn hash(bytes: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub fn input_path(root: &Path, relative: &str) -> Result<PathBuf> {
    let path = Path::new(relative);
    if relative.is_empty()
        || relative.contains('\\')
        || path.is_absolute()
        || path
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err(format!("invalid portable workspace input: {relative:?}").into());
    }
    let mut cursor = root.to_path_buf();
    for part in path.components() {
        cursor.push(part);
        let metadata = std::fs::symlink_metadata(&cursor)?;
        let linked = metadata.file_type().is_symlink();
        #[cfg(windows)]
        let linked = {
            use std::os::windows::fs::MetadataExt;
            linked || metadata.file_attributes() & 0x400 != 0
        };
        if linked {
            return Err(format!("linked input is not permitted: {relative}").into());
        }
    }
    Ok(cursor)
}
