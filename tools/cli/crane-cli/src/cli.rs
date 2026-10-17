use std::path::PathBuf;

use clap::{
    Args,
    Parser,
    Subcommand,
};

use crate::CraneError;

#[derive(Debug, Parser)]
#[command(
    name = "crane",
    about = "Filter git history for selected source trees and transplant it into another repository",
    version,
    arg_required_else_help = true
)]
pub struct CraneCli {
    #[command(subcommand)]
    pub command: CraneCommand,
}

#[derive(Debug, Subcommand)]
pub enum CraneCommand {
    Transplant(TransplantArgs),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathMapping {
    pub source: String,
    pub destination: String,
}

#[derive(Debug, Args, Clone)]
pub struct TransplantArgs {
    #[arg(long)]
    pub source_repo: PathBuf,

    #[arg(long)]
    pub target_repo: PathBuf,

    #[arg(long, default_value = "HEAD")]
    pub source_ref: String,

    #[arg(long, default_value = "main")]
    pub target_branch: String,

    #[arg(long, default_value = "crane/import")]
    pub import_branch: String,

    #[arg(long)]
    pub anchor_commit: Option<String>,

    #[arg(
        long = "mapping",
        value_parser = parse_mapping,
        required = true,
        help = "Map <source>=<destination>; use an empty destination (for example crates/context-stack=) to rewrite the selected subtree to branch root"
    )]
    pub mappings: Vec<PathMapping>,

    #[arg(long, default_value_t = false)]
    pub no_merge: bool,

    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

pub fn parse_mapping(raw: &str) -> Result<PathMapping, String> {
    let Some((source, destination)) = raw.split_once('=') else {
        return Err(format!(
            "invalid mapping `{raw}`; expected <source>=<destination>"
        ));
    };

    let source = normalize_path(source);
    let destination = normalize_path(destination);

    if source.is_empty() {
        return Err(format!("invalid mapping `{raw}`; source path is empty"));
    }

    Ok(PathMapping {
        source,
        destination,
    })
}

pub fn normalize_path(raw: &str) -> String {
    raw.replace('\\', "/")
        .trim()
        .trim_start_matches("./")
        .trim_matches('/')
        .to_string()
}

pub fn validate_mappings(
    mappings: &[PathMapping]
) -> Result<Vec<PathMapping>, CraneError> {
    let mut normalized = mappings.to_vec();
    normalized
        .sort_by(|left, right| right.source.len().cmp(&left.source.len()));

    for (index, left) in normalized.iter().enumerate() {
        for right in normalized.iter().skip(index + 1) {
            if left.source == right.source {
                return Err(CraneError::BadRequest(format!(
                    "duplicate source mapping for `{}`",
                    left.source
                )));
            }
            if is_path_prefix(&left.source, &right.source)
                || is_path_prefix(&right.source, &left.source)
            {
                return Err(CraneError::BadRequest(format!(
                    "overlapping source mappings are not supported: `{}` and `{}`",
                    left.source, right.source
                )));
            }
            if path_scopes_overlap(&left.destination, &right.destination) {
                return Err(CraneError::BadRequest(format!(
                    "overlapping destination mappings are not supported: `{}={}` and `{}={}`",
                    left.source,
                    left.destination,
                    right.source,
                    right.destination
                )));
            }
        }
    }

    Ok(normalized)
}

fn path_scopes_overlap(
    left: &str,
    right: &str,
) -> bool {
    if left.is_empty() || right.is_empty() {
        return true;
    }

    is_path_prefix(left, right) || is_path_prefix(right, left)
}

fn is_path_prefix(
    parent: &str,
    child: &str,
) -> bool {
    child == parent
        || child
            .strip_prefix(parent)
            .is_some_and(|suffix| suffix.starts_with('/'))
}
