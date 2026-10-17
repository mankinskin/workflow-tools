mod cli;
mod git;
mod transform;
mod transplant;

#[cfg(test)]
mod tests;

pub use cli::{
    CraneCli,
    CraneCommand,
    PathMapping,
    TransplantArgs,
};
pub use transplant::{
    TransplantOutcome,
    execute,
};

use std::{
    path::PathBuf,
    string::FromUtf8Error,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CraneError {
    #[error("{0}")]
    BadRequest(String),
    #[error("missing history for selected paths: {0}")]
    MissingPathHistory(String),
    #[error("target repository is not clean: {0}")]
    DirtyTargetRepo(PathBuf),
    #[error(
        "git command failed in {cwd}: `{command}` (status: {status:?}) {stderr}"
    )]
    CommandFailed {
        cwd: PathBuf,
        command: String,
        status: Option<i32>,
        stderr: String,
    },
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("utf-8 error: {0}")]
    Utf8(#[from] FromUtf8Error),
    #[error("fast-export transform error: {0}")]
    FastExport(String),
}

pub fn run(cli: CraneCli) -> Result<String, CraneError> {
    match cli.command {
        CraneCommand::Transplant(args) => {
            let outcome = transplant::execute(args)?;
            Ok(render_outcome(&outcome))
        },
    }
}

fn render_outcome(outcome: &TransplantOutcome) -> String {
    let mut lines = vec![
        format!("source_repo={}", outcome.plan.source_repo.display()),
        format!("target_repo={}", outcome.plan.target_repo.display()),
        format!("source_ref={}", outcome.plan.source_ref),
        format!("source_commit={}", outcome.plan.source_commit),
        format!("anchor_commit={}", outcome.plan.anchor_commit),
        format!("range_spec={}", outcome.plan.range_spec),
        format!("target_branch={}", outcome.plan.target_branch),
        format!("import_branch={}", outcome.plan.import_branch),
        format!("import_ref={}", outcome.plan.import_ref),
        format!("merged={}", outcome.merged),
    ];

    for mapping in &outcome.plan.mappings {
        lines.push(format!(
            "mapping={}={}",
            mapping.source, mapping.destination
        ));
    }

    if let Some(stats) = &outcome.stats {
        lines.push(format!("commit_count={}", stats.commit_count));
        lines.push(format!("blob_count={}", stats.blob_count));
        lines.push(format!("rewritten_ops={}", stats.rewritten_ops));
        lines.push(format!("dropped_ops={}", stats.dropped_ops));
    } else {
        lines.push("dry_run=true".to_string());
    }

    lines.join("\n")
}
