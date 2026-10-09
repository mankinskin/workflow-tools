pub mod gates;
pub mod inventory;
pub mod model;
pub mod reconcile;

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

fn allowed_root(root: &Path) -> Result<(PathBuf, Option<PathBuf>)> {
    let root = root.canonicalize()?;
    let results = std::env::var_os("WORKFLOW_VALIDATION_RESULTS")
        .map(PathBuf::from)
        .map(|path| path.canonicalize())
        .transpose()?;
    Ok((root, results))
}

fn path_has_parent_traversal(path: &Path) -> bool {
    path.components()
        .any(|part| matches!(part, Component::ParentDir))
}

fn allowed_base(
    canonical: &Path,
    root: &Path,
    results: Option<&Path>,
    requested: &Path,
) -> Result<PathBuf> {
    if canonical.starts_with(root) {
        return Ok(root.to_path_buf());
    }
    if let Some(results) = results.filter(|results| canonical.starts_with(results)) {
        return Ok(results.to_path_buf());
    }
    Err(format!("path escapes validation roots: {}", requested.display()).into())
}

fn reject_link_components(base: &Path, canonical: &Path, requested: &Path) -> Result<()> {
    let relative = canonical.strip_prefix(base)?;
    let mut cursor = base.to_path_buf();
    for component in relative.components() {
        if !matches!(component, Component::Normal(_)) {
            return Err(format!("invalid path: {}", requested.display()).into());
        }
        cursor.push(component);
        if std::fs::symlink_metadata(&cursor)?.file_type().is_symlink() {
            return Err(format!("linked path is not permitted: {}", requested.display()).into());
        }
    }
    Ok(())
}

pub fn validation_input_path(root: &Path, requested: &Path) -> Result<PathBuf> {
    if path_has_parent_traversal(requested) {
        return Err(format!(
            "input path contains parent traversal: {}",
            requested.display()
        )
        .into());
    }
    let (root, results) = allowed_root(root)?;
    let path = if requested.is_absolute() {
        requested.to_path_buf()
    } else {
        root.join(requested)
    };
    let canonical = path.canonicalize()?;
    let base = allowed_base(&canonical, &root, results.as_deref(), requested)?;
    reject_link_components(&base, &canonical, requested)?;
    Ok(canonical)
}

pub fn output_path(root: &Path, requested: &Path) -> Result<PathBuf> {
    if path_has_parent_traversal(requested) {
        return Err(format!(
            "output path contains parent traversal: {}",
            requested.display()
        )
        .into());
    }
    let (root, results) = allowed_root(root)?;
    let candidate = if requested.is_absolute() {
        requested.to_path_buf()
    } else {
        root.join(requested)
    };
    let parent = candidate
        .parent()
        .ok_or_else(|| format!("output has no parent: {}", candidate.display()))?
        .canonicalize()?;
    let base = allowed_base(&parent, &root, results.as_deref(), requested)?;
    reject_link_components(&base, &parent, requested)?;
    let output = parent.join(
        candidate
            .file_name()
            .ok_or_else(|| format!("output has no filename: {}", candidate.display()))?,
    );
    if output.exists() && std::fs::symlink_metadata(&output)?.file_type().is_symlink() {
        return Err(format!("linked output is not permitted: {}", output.display()).into());
    }
    Ok(output)
}
