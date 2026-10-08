use clap::{Parser, Subcommand, ValueEnum};
use path_render_inventory::{
    Result,
    gates::{self, PlatformKey},
    inventory,
    model::*,
};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

#[derive(Parser)]
#[command(about = "Conservative path-rendering census and stage-specific campaign gates")]
struct Cli {
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand)]
enum Action {
    Scan {
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        source: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Batches {
        #[arg(long)]
        ledger: PathBuf,
        #[arg(long, value_enum)]
        stage: StageArg,
        #[arg(long)]
        windows: bool,
    },
    Validate {
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        source: PathBuf,
        #[arg(long)]
        ledger: PathBuf,
        #[arg(long)]
        selector: String,
        #[arg(long, value_enum)]
        stage: StageArg,
        #[arg(long, value_enum)]
        platform: PlatformArg,
        #[arg(long)]
        receipts: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long)]
        run_selected_tests: bool,
        #[arg(long)]
        check_ledger: bool,
        #[arg(long)]
        verify_complete: bool,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum StageArg {
    Inventory,
    Migration,
    Final,
}
impl From<StageArg> for Stage {
    fn from(value: StageArg) -> Self {
        match value {
            StageArg::Inventory => Self::Inventory,
            StageArg::Migration => Self::Migration,
            StageArg::Final => Self::Final,
        }
    }
}
#[derive(Clone, Copy, ValueEnum)]
enum PlatformArg {
    Linux,
    Windows,
}
impl From<PlatformArg> for Platform {
    fn from(value: PlatformArg) -> Self {
        match value {
            PlatformArg::Linux => Self::Linux,
            PlatformArg::Windows => Self::Windows,
        }
    }
}

fn read_ledger(path: &Path) -> Result<Ledger> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}

fn read_proofs(directory: &Path) -> Result<Vec<OwnerProof>> {
    let mut proofs = vec![];
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path.extension().is_none_or(|extension| extension != "json") {
            continue;
        }
        let value: serde_json::Value = serde_json::from_slice(&fs::read(&path)?)?;
        if value.get("status").and_then(|value| value.as_str()) != Some("passed") {
            continue;
        }
        if let Some(rows) = value.get("owner_proofs") {
            let image = value
                .get("image")
                .and_then(|value| value.as_str())
                .ok_or("owner receipt has no Docker image")?;
            if image
                .strip_prefix("sha256:")
                .is_none_or(|id| id.len() != 64 || !id.bytes().all(|byte| byte.is_ascii_hexdigit()))
            {
                return Err("owner receipt has invalid Docker image attestation".into());
            }
            let platform: Platform = serde_json::from_value(
                value
                    .get("platform")
                    .ok_or("owner receipt has no platform")?
                    .clone(),
            )?;
            let rows: Vec<OwnerProof> = serde_json::from_value(rows.clone())?;
            if rows.iter().any(|row| row.platform != platform) {
                return Err("receipt/owner platform mismatch".into());
            }
            proofs.extend(rows);
        }
    }
    Ok(proofs)
}

fn checked_cargo(root: &Path, args: &[String]) -> Result<()> {
    let status = Command::new("cargo")
        .current_dir(root)
        .args(args)
        .status()?;
    if !status.success() {
        return Err(format!("Cargo validation failed ({status}): {args:?}").into());
    }
    Ok(())
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Action::Scan {
            root,
            source,
            output,
        } => {
            if output.exists() {
                return Err(
                    "scan output already exists; retain the old ledger and reconcile explicitly"
                        .into(),
                );
            }
            let source: Source = serde_json::from_slice(&fs::read(source)?)?;
            let ledger = inventory::scan(&root, &source)?;
            fs::write(output, toml::to_string_pretty(&ledger)?)?;
            println!(
                "Pending inventory: {} repositories, {} Rust files, {} owners",
                ledger.repositories.len(),
                ledger.files.len(),
                ledger.owners.len()
            );
        }
        Action::Batches {
            ledger,
            stage,
            windows,
        } => {
            let ledger = read_ledger(&ledger)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&inventory::selections(
                    &ledger,
                    stage.into(),
                    windows
                )?)?
            );
        }
        Action::Validate {
            root,
            source,
            ledger,
            selector,
            stage,
            platform,
            receipts,
            output,
            run_selected_tests,
            check_ledger,
            verify_complete,
        } => {
            if std::env::var("WORKFLOW_VALIDATION_CONTAINER")
                .ok()
                .as_deref()
                != Some("platform")
            {
                return Err("validation commands require the common container driver".into());
            }
            let platform = Platform::from(platform);
            if run_selected_tests && (platform == Platform::Windows) != cfg!(windows) {
                return Err("requested platform does not match native container OS".into());
            }
            if !run_selected_tests && !check_ledger && !verify_complete {
                return Err("no validation action requested".into());
            }
            let source: Source = serde_json::from_slice(&fs::read(source)?)?;
            let ledger = read_ledger(&ledger)?;
            gates::check_schema(&ledger, &source)?;
            gates::check_detected_coverage(&ledger, &inventory::scan(&root, &source)?)?;
            let selection = gates::select(&ledger, &selector, stage.into())?;
            if verify_complete && !selection.aggregate {
                return Err("VerifyComplete requires an aggregate selector".into());
            }
            gates::check_classification(&ledger, &source, &selection)?;
            let mut proofs = read_proofs(&receipts)?;
            let mut current = BTreeMap::new();
            let mut owners = vec![];
            let parent = output
                .parent()
                .ok_or("validation output needs a parent directory")?;
            for owner in ledger.owners.iter().filter(|owner| {
                selection.owner_ids.contains(&owner.id)
                    && (run_selected_tests || selection.stage != Stage::Inventory)
            }) {
                let response = Command::new("cargo")
                    .current_dir(&root)
                    .args([
                        "metadata",
                        "--locked",
                        "--format-version",
                        "1",
                        "--manifest-path",
                        &owner.manifest,
                    ])
                    .output()?;
                if !response.status.success() {
                    return Err(format!(
                        "owner metadata failed: {}: {}",
                        owner.package,
                        String::from_utf8_lossy(&response.stderr)
                    )
                    .into());
                }
                let graph: CargoGraph = serde_json::from_slice(&response.stdout)?;
                if selection.stage == Stage::Final {
                    gates::check_consumer_pin(
                        &ledger,
                        owner,
                        &root,
                        Path::new(&graph.workspace_root),
                    )?;
                }
                let mut platforms = vec![if selection.selector.starts_with("windows-batch:") {
                    platform
                } else {
                    Platform::Linux
                }];
                if selection.selector == "final-both-platform-complete"
                    && owner.windows == WindowsApplicability::Applicable
                {
                    platforms.push(Platform::Windows)
                }
                for expected_platform in platforms {
                    let digest =
                        gates::owner_digest(owner, &graph, &source, &root, expected_platform)?;
                    current.insert(
                        (owner.id.clone(), PlatformKey::from(expected_platform)),
                        digest,
                    );
                }
                if run_selected_tests {
                    let recipe = match platform {
                        Platform::Linux => &owner.linux_recipe,
                        Platform::Windows => &owner.windows_recipe,
                    };
                    let expected_recipe = match platform {
                        Platform::Linux => "rust-bookworm-openssl",
                        Platform::Windows => "rust-servercore-ltsc2022-msvc",
                    };
                    if recipe != expected_recipe {
                        return Err(format!("unsupported owner dependency recipe: {recipe}").into());
                    }
                    let args = gates::test_arguments(owner, platform)?.to_vec();
                    checked_cargo(&root, &args)?;
                    let metadata_file = format!("cargo-metadata-{}.json", owner.id);
                    fs::write(parent.join(&metadata_file), &response.stdout)?;
                    let proof = OwnerProof {
                        owner: owner.id.clone(),
                        platform,
                        content_digest: gates::owner_digest(
                            owner, &graph, &source, &root, platform,
                        )?,
                        test_args: args,
                        recipe: recipe.clone(),
                        passed: true,
                    };
                    proofs.push(proof.clone());
                    owners.push(OwnerRun {
                        owner: owner.id.clone(),
                        metadata_file,
                        proof,
                    });
                }
            }
            gates::check_platform_evidence(&ledger, &selection, &current, &proofs, platform)?;
            fs::write(
                output,
                serde_json::to_vec_pretty(&ValidationResult {
                    selection,
                    owners,
                    publication_revision: ledger
                        .publication
                        .as_ref()
                        .map(|publication| publication.revision.clone()),
                })?,
            )?;
            println!("Stage gate passed: {selector}");
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("path-render-inventory: {error}");
            ExitCode::FAILURE
        }
    }
}
