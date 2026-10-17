use std::{
    io::{
        BufReader,
        BufWriter,
    },
    path::PathBuf,
    process::Stdio,
};

use crate::{
    CraneError,
    PathMapping,
    TransplantArgs,
    cli::validate_mappings,
    git::{
        GitRepo,
        normalize_branch_ref,
    },
    transform::{
        TransformStats,
        transform_export,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransplantPlan {
    pub source_repo: PathBuf,
    pub target_repo: PathBuf,
    pub source_ref: String,
    pub source_commit: String,
    pub anchor_commit: String,
    pub range_spec: String,
    pub target_branch: String,
    pub import_branch: String,
    pub import_ref: String,
    pub mappings: Vec<PathMapping>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransplantOutcome {
    pub plan: TransplantPlan,
    pub merged: bool,
    pub stats: Option<TransformStats>,
}

pub fn execute(args: TransplantArgs) -> Result<TransplantOutcome, CraneError> {
    let source_repo = GitRepo::open(&args.source_repo)?;
    let target_repo = GitRepo::open(&args.target_repo)?;
    source_repo.assert_repo()?;
    target_repo.assert_repo()?;

    let mappings = validate_mappings(&args.mappings)?;
    let source_commit = source_repo.rev_parse(&args.source_ref)?;
    let anchor_commit = match args.anchor_commit.as_deref() {
        Some(commit) => source_repo.rev_parse(commit)?,
        None => {
            let source_paths = mappings
                .iter()
                .map(|mapping| mapping.source.clone())
                .collect::<Vec<_>>();
            source_repo
                .first_touching_commit(&source_commit, &source_paths)?
                .ok_or_else(|| {
                    CraneError::MissingPathHistory(source_paths.join(", "))
                })?
        },
    };

    let anchor_parent = source_repo.rev_parse(&format!("{anchor_commit}^"));
    let range_spec = match anchor_parent {
        Ok(parent) => format!("{parent}..{source_commit}"),
        Err(_) => source_commit.clone(),
    };
    let import_ref = normalize_branch_ref(&args.import_branch);

    let plan = TransplantPlan {
        source_repo: source_repo.path().to_path_buf(),
        target_repo: target_repo.path().to_path_buf(),
        source_ref: args.source_ref,
        source_commit,
        anchor_commit,
        range_spec,
        target_branch: args.target_branch,
        import_branch: args.import_branch,
        import_ref,
        mappings,
    };

    if args.dry_run {
        return Ok(TransplantOutcome {
            plan,
            merged: false,
            stats: None,
        });
    }

    if !target_repo.is_clean()? {
        return Err(CraneError::DirtyTargetRepo(
            target_repo.path().to_path_buf(),
        ));
    }

    target_repo.delete_ref_if_exists(&plan.import_ref)?;

    let stats = import_history(&source_repo, &target_repo, &plan)?;

    if !args.no_merge {
        target_repo.checkout(&plan.target_branch)?;
        target_repo.merge_allow_unrelated(
            &plan.import_branch,
            &format!(
                "Import filtered history from {} via crane-cli",
                plan.source_repo.display()
            ),
        )?;
    }

    Ok(TransplantOutcome {
        plan,
        merged: !args.no_merge,
        stats: Some(stats),
    })
}

fn import_history(
    source_repo: &GitRepo,
    target_repo: &GitRepo,
    plan: &TransplantPlan,
) -> Result<TransformStats, CraneError> {
    let mut export_args = vec![
        "fast-export".to_string(),
        "--show-original-ids".to_string(),
        "--signed-tags=strip".to_string(),
        "--tag-of-filtered-object=drop".to_string(),
        plan.range_spec.clone(),
        "--".to_string(),
    ];
    export_args
        .extend(plan.mappings.iter().map(|mapping| mapping.source.clone()));

    let mut exporter = source_repo
        .command(export_args.clone())
        .stdout(Stdio::piped())
        .stdin(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(CraneError::Io)?;

    let mut importer = target_repo
        .command(["fast-import", "--force"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(CraneError::Io)?;

    let export_stdout = exporter.stdout.take().ok_or_else(|| {
        CraneError::FastExport(
            "failed to capture fast-export stdout".to_string(),
        )
    })?;
    let import_stdin = importer.stdin.take().ok_or_else(|| {
        CraneError::FastExport("failed to open fast-import stdin".to_string())
    })?;

    let stats = transform_export(
        BufReader::new(export_stdout),
        BufWriter::new(import_stdin),
        &plan.import_ref,
        &plan.mappings,
    );

    if let Err(error) = stats {
        let _ = exporter.kill();
        let _ = importer.kill();
        return Err(error);
    }
    let stats = stats.expect("stats already checked above");

    let export_status = exporter.wait().map_err(CraneError::Io)?;
    if !export_status.success() {
        return Err(CraneError::CommandFailed {
            cwd: source_repo.path().to_path_buf(),
            command: format!("git {}", export_args.join(" ")),
            status: export_status.code(),
            stderr: String::new(),
        });
    }

    let import_status = importer.wait().map_err(CraneError::Io)?;
    if !import_status.success() {
        return Err(CraneError::CommandFailed {
            cwd: target_repo.path().to_path_buf(),
            command: "git fast-import --force".to_string(),
            status: import_status.code(),
            stderr: String::new(),
        });
    }

    Ok(stats)
}
