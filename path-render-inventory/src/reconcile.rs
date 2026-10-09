use crate::{
    Result, hash,
    gates,
    inventory,
    model::*,
    output_path,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs,
    path::{Path, PathBuf},
};

pub const CURRENT_SCHEMA: u32 = 2;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReconciliationEvent {
    pub kind: String,
    pub id: String,
    pub action: String,
    pub reason: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CandidateFile {
    pub id: String,
    pub path: String,
    pub source_digest: String,
    pub origin: String,
    pub owner: Option<String>,
    pub occurrence_ids: Vec<String>,
    pub gap_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CandidateReport {
    pub schema: u32,
    pub source_digest: String,
    pub repository_paths: Vec<String>,
    pub files: Vec<CandidateFile>,
    pub file_count: usize,
    pub occurrence_count: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReconciliationReport {
    pub schema: u32,
    pub operation: String,
    pub ledger_digest: String,
    pub source_digest: String,
    pub previous_source_digest: Option<String>,
    pub previous_schema: Option<u32>,
    pub live_files: usize,
    pub tombstones: usize,
    pub events: Vec<ReconciliationEvent>,
    pub invalidated_owner_ids: Vec<String>,
    pub candidates: CandidateReport,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HandoffExport {
    pub schema: u32,
    pub current_stage: Stage,
    pub ledger_digest: String,
    pub source_digest: String,
    pub selector_digest: String,
    pub inventory: Vec<Selection>,
    pub migration: Vec<Selection>,
    pub windows: Vec<Selection>,
    pub pending_files: usize,
    pub pending_occurrences: usize,
    pub pending_deletions: usize,
    pub cap_checks: Vec<CapCheck>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CapCheck {
    pub selector: String,
    pub files: usize,
    pub occurrences: usize,
    pub deletions: usize,
    pub passed: bool,
}

fn path_is_within(path: &str, directory: &str) -> bool {
    path == directory || path.starts_with(&format!("{directory}/"))
}

fn repository_digest(source: &Source, repository: &str) -> String {
    let mut inputs: Vec<_> = source
        .files
        .iter()
        .filter(|file| {
            (repository.is_empty() || path_is_within(&file.path, repository))
                && !file.path.ends_with(".rs")
        })
        .map(|file| format!("file\t{}\t{}", file.path, file.hash))
        .chain(
            source
                .files
                .iter()
                .filter(|file| {
                    (repository.is_empty() || path_is_within(&file.path, repository))
                        && file.path.ends_with(".rs")
                })
                .map(|file| format!("rust\t{}\t{}", file.path, file.hash)),
        )
        .chain(
            source
                .exclusions
                .iter()
                .filter(|file| {
                    repository.is_empty() || path_is_within(&file.path, repository)
                })
                .map(|file| format!("excluded\t{}\t{}", file.path, file.reason)),
        )
        .collect();
    inputs.sort();
    hash(inputs.join("\n"))
}

fn owner_fingerprint(owner: &Owner, source: &Source) -> OwnerFingerprint {
    let package_root = Path::new(&owner.package_manifest)
        .parent()
        .and_then(Path::to_str)
        .unwrap_or("");
    let mut directory = Path::new(&owner.package_manifest).parent();
    let lock_file = loop {
        let Some(parent) = directory else {
            break None;
        };
        let candidate = parent.join("Cargo.lock");
        if let Some(file) = candidate
            .to_str()
            .and_then(|path| source.files.iter().find(|file| file.path == path))
        {
            break Some(file);
        }
        directory = parent.parent();
    };
    let lock_packages = lock_file
        .map(|file| file.cargo_packages.as_slice())
        .unwrap_or_default();
    let roots: Vec<_> = lock_packages
        .iter()
        .filter(|package| package.name == owner.package && package.version == owner.version)
        .collect();
    let mut lock_inputs = Vec::new();
    let mut visited = BTreeSet::new();
    let mut pending: VecDeque<_> = roots.into_iter().collect();
    let mut lock_verified = !pending.is_empty();
    while let Some(package) = pending.pop_front() {
        let identity = (package.name.clone(), package.version.clone());
        if !visited.insert(identity.clone()) {
            continue;
        }
        if !lock_verified {
            lock_inputs.clear();
            if let Some(lock_file) = lock_file {
                lock_inputs.push(format!("lockfile\t{}", lock_file.hash));
                lock_verified = true;
            }
        }
        lock_inputs.push(format!(
            "{}@{}\t{}",
            package.name, package.version, package.hash
        ));
        for dependency in &package.dependencies {
            let mut parts = dependency.split_whitespace();
            let name = parts.next().unwrap_or_default();
            let version = parts.next().filter(|part| {
                part.chars()
                    .next()
                    .is_some_and(|character| character.is_ascii_digit())
            });
            let matching: Vec<_> = lock_packages
                .iter()
                .filter(|candidate| {
                    candidate.name == name
                        && version.is_none_or(|version| candidate.version == version)
                })
                .collect();
            if matching.is_empty() {
                lock_verified = false;
            } else {
                pending.extend(matching);
            }
        }
    }
    let mut manifest_inputs: Vec<_> = source
        .files
        .iter()
        .filter(|file| file.path == owner.manifest || file.path == owner.package_manifest)
        .map(|file| format!("{}\t{}", file.path, file.hash))
        .chain(lock_inputs)
        .collect();
    manifest_inputs.sort();
    let manifests_verified = lock_verified
        && [owner.manifest.as_str(), owner.package_manifest.as_str()]
            .into_iter()
            .all(|path| source.files.iter().any(|file| file.path == path));
    let mut test_inputs: Vec<_> = source
        .files
        .iter()
        .filter(|file| {
            file.path.ends_with(".rs")
                && (package_root.is_empty()
                    || path_is_within(&file.path, package_root))
        })
        .map(|file| format!("{}\t{}", file.path, file.hash))
        .collect();
    test_inputs.sort();
    let tests_verified = !test_inputs.is_empty();
    let recipe = |platform: &str| {
        source
            .recipe_digests
            .get(platform)
            .cloned()
            .unwrap_or_default()
    };
    OwnerFingerprint {
        manifest_digest: if manifests_verified {
            hash(manifest_inputs.join("\n"))
        } else {
            String::new()
        },
        tests_digest: if tests_verified {
            hash(test_inputs.join("\n"))
        } else {
            String::new()
        },
        test_args_digest: hash(
            serde_json::to_vec(&owner.test_args).expect("serializing owner arguments is infallible"),
        ),
        linux_recipe_digest: recipe("linux"),
        windows_recipe_digest: recipe("windows"),
    }
}

pub fn populate_fingerprints(ledger: &mut Ledger, source: &Source) {
    ledger.file_fingerprints = ledger
        .files
        .iter()
        .map(|file| {
            (
                file.path.clone(),
                FileFingerprint {
                    source_digest: file.source_digest.clone(),
                    origin: file.origin.clone(),
                    owner: file.owner.clone(),
                },
            )
        })
        .collect();
    ledger.repository_fingerprints = ledger
        .repositories
        .iter()
        .map(|repository| {
            (
                repository.path.clone(),
                repository_digest(source, &repository.path),
            )
        })
        .collect();
    ledger.owner_fingerprints = ledger
        .owners
        .iter()
        .map(|owner| (owner.id.clone(), owner_fingerprint(owner, source)))
        .collect();
}

fn candidate_report(ledger: &Ledger) -> CandidateReport {
    let files: Vec<_> = ledger
        .files
        .iter()
        .map(|file| CandidateFile {
            id: file.id.clone(),
            path: file.path.clone(),
            source_digest: file.source_digest.clone(),
            origin: file.origin.clone(),
            owner: file.owner.clone(),
            occurrence_ids: file
                .occurrences
                .iter()
                .map(|occurrence| occurrence.id.clone())
                .collect(),
            gap_ids: file.gaps.iter().map(|gap| gap.id.clone()).collect(),
        })
        .collect();
    CandidateReport {
        schema: 1,
        source_digest: ledger.source_digest.clone(),
        repository_paths: ledger
            .repositories
            .iter()
            .map(|repository| repository.path.clone())
            .collect(),
        file_count: files.len(),
        occurrence_count: files.iter().map(|file| file.occurrence_ids.len()).sum(),
        files,
    }
}

fn clear_windows(owner: &mut Owner) {
    owner.windows = WindowsApplicability::Pending;
    owner.windows_test_args.clear();
    owner.windows_recipe.clear();
    owner.windows_rationale.clear();
}

fn event(
    events: &mut Vec<ReconciliationEvent>,
    kind: &str,
    id: impl Into<String>,
    action: &str,
    reason: &str,
) {
    events.push(ReconciliationEvent {
        kind: kind.into(),
        id: id.into(),
        action: action.into(),
        reason: reason.into(),
    });
}

pub fn reconcile(root: &Path, source: &Source, old: &Ledger) -> Result<(Ledger, ReconciliationReport)> {
    if !matches!(old.schema, 1 | CURRENT_SCHEMA) {
        return Err(format!("cannot explicitly migrate ledger schema {}", old.schema).into());
    }
    let previous_schema = old.schema;
    let mut fresh = inventory::scan(root, source)?;
    let mut events = vec![];
    let mut invalidated_owners = BTreeSet::new();
    if previous_schema == 1 {
        event(
            &mut events,
            "schema",
            "ledger",
            "migrated",
            "schema 1 accepted explicitly; provenance must be established before reviews can be retained",
        );
    }

    let old_files: BTreeMap<_, _> = old.files.iter().map(|file| (file.path.clone(), file)).collect();
    let fresh_paths: BTreeSet<_> = fresh.files.iter().map(|file| file.path.clone()).collect();
    for file in &mut fresh.files {
        let Some(previous) = old_files.get(&file.path) else {
            event(
                &mut events,
                "file",
                &file.path,
                "added",
                "new source input has no prior review",
            );
            continue;
        };
        let current = FileFingerprint {
            source_digest: file.source_digest.clone(),
            origin: file.origin.clone(),
            owner: file.owner.clone(),
        };
        let fingerprint_matches = previous_schema == CURRENT_SCHEMA
            && old.file_fingerprints.get(&file.path) == Some(&current)
            && previous.source_digest == current.source_digest
            && previous.origin == current.origin
            && previous.owner == current.owner;
        if !fingerprint_matches {
            event(
                &mut events,
                "file",
                &file.path,
                "reset",
                if previous_schema == 1 {
                    "legacy schema has no verified file provenance"
                } else {
                    "source hash, origin, owner, or stored fingerprint changed"
                },
            );
            if let Some(owner) = &previous.owner {
                invalidated_owners.insert(owner.clone());
            }
            if let Some(owner) = &file.owner {
                invalidated_owners.insert(owner.clone());
            }
            continue;
        }

        let mut retained = (*previous).clone();
        let known: BTreeSet<_> = retained
            .occurrences
            .iter()
            .map(|row| row.id.clone())
            .collect();
        for candidate in &file.occurrences {
            if !known.contains(&candidate.id) {
                retained.occurrences.push(candidate.clone());
                event(
                    &mut events,
                    "occurrence",
                    &candidate.id,
                    "added",
                    "current source scan found a candidate absent from the prior review",
                );
            }
        }
        let known_gaps: BTreeSet<_> = retained.gaps.iter().map(|gap| gap.id.clone()).collect();
        for gap in &file.gaps {
            if !known_gaps.contains(&gap.id) {
                retained.gaps.push(gap.clone());
                event(
                    &mut events,
                    "gap",
                    &gap.id,
                    "added",
                    "current source scan found a coverage gap absent from the prior review",
                );
            }
        }
        *file = retained;
        event(
            &mut events,
            "file",
            &file.path,
            "retained",
            "verified source, origin, and owner fingerprint is unchanged",
        );
    }

    let existing_tombstones: BTreeMap<_, _> = old
        .tombstones
        .iter()
        .map(|tombstone| (tombstone.id.clone(), tombstone.clone()))
        .collect();
    let mut tombstones: BTreeMap<_, _> = existing_tombstones.clone();
    for previous in old.files.iter().filter(|file| !fresh_paths.contains(&file.path)) {
        let id = hash(format!(
            "deleted-source\n{}\n{}",
            previous.path, previous.source_digest
        ));
        let id = id[..24].to_owned();
        if !tombstones.contains_key(&id) {
            let tombstone = DeletionReview {
                id: id.clone(),
                file: previous.clone(),
                reason: "Source file disappeared from the recursive census; human deletion review is required.".into(),
                resolution: String::new(),
            };
            tombstones.insert(id.clone(), tombstone);
            event(
                &mut events,
                "deletion",
                id,
                "added",
                "deleted source retained as an unresolved audit tombstone",
            );
            if let Some(owner) = &previous.owner {
                invalidated_owners.insert(owner.clone());
            }
        }
    }
    for tombstone in old.tombstones.iter() {
        if previous_schema == 1 {
            event(
                &mut events,
                "deletion",
                &tombstone.id,
                "reset",
                "legacy deletion resolution has no verified provenance",
            );
            let mut migrated = tombstone.clone();
            migrated.resolution.clear();
            tombstones.insert(migrated.id.clone(), migrated);
        }
    }

    let old_repositories: BTreeMap<_, _> = old
        .repositories
        .iter()
        .map(|repository| (repository.path.clone(), repository))
        .collect();
    for repository in &mut fresh.repositories {
        let current_digest = repository_digest(source, &repository.path);
        let previous = old_repositories.get(&repository.path);
        let verified = previous_schema == CURRENT_SCHEMA
            && old.repository_fingerprints.get(&repository.path) == Some(&current_digest)
            && previous.is_some_and(|record| record.group == repository.group);
        if verified {
            let previous = previous.expect("verified repository has a prior record");
            repository.reviewed = previous.reviewed;
            repository.rationale = previous.rationale.clone();
            event(
                &mut events,
                "repository",
                &repository.path,
                "retained",
                "verified repository census fingerprint is unchanged",
            );
        } else {
            repository.reviewed = false;
            repository.rationale.clear();
            event(
                &mut events,
                "repository",
                &repository.path,
                if previous.is_some() { "reset" } else { "added" },
                "repository census fingerprint is missing or changed",
            );
        }
    }

    let current_owner_fingerprints: BTreeMap<_, _> = fresh
        .owners
        .iter()
        .map(|owner| (owner.id.clone(), owner_fingerprint(owner, source)))
        .collect();
    let old_owners: BTreeMap<_, _> = old
        .owners
        .iter()
        .map(|owner| (owner.id.clone(), owner))
        .collect();
    for owner in &mut fresh.owners {
        let current = current_owner_fingerprints
            .get(&owner.id)
            .expect("fingerprint built from current owner");
        let old_fingerprint = old.owner_fingerprints.get(&owner.id);
        let old_owner = old_owners.get(&owner.id);
        let previous_owner_evidence = old_owner.is_some_and(|previous| {
            previous.windows != WindowsApplicability::Pending
                || !previous.windows_test_args.is_empty()
                || !previous.windows_recipe.is_empty()
                || !previous.windows_rationale.is_empty()
                || previous.dependency_revision.is_some()
        });
        let previous_windows_evidence = old_owner.is_some_and(|previous| {
            previous.windows != WindowsApplicability::Pending
                || !previous.windows_test_args.is_empty()
                || !previous.windows_recipe.is_empty()
                || !previous.windows_rationale.is_empty()
        });
        let base_matches = previous_schema == CURRENT_SCHEMA
            && old_fingerprint.is_some_and(|fingerprint| {
                fingerprint.manifest_digest == current.manifest_digest
                    && fingerprint.tests_digest == current.tests_digest
                    && fingerprint.test_args_digest == current.test_args_digest
                    && fingerprint.linux_recipe_digest == current.linux_recipe_digest
                    && !current.manifest_digest.is_empty()
                    && !current.tests_digest.is_empty()
                    && !current.linux_recipe_digest.is_empty()
            });
        if base_matches {
            *owner = (**old_owner.expect("matching fingerprint has a prior owner")).clone();
        } else {
            clear_windows(owner);
            owner.dependency_revision = None;
            if previous_owner_evidence {
                invalidated_owners.insert(owner.id.clone());
            }
            event(
                &mut events,
                "owner",
                &owner.id,
                if old_owner.is_some() { "reset" } else { "added" },
                "manifest, package sources, test arguments, Linux recipe, or owner fingerprint is missing/changed",
            );
        }
        let windows_matches = base_matches
            && !current.windows_recipe_digest.is_empty()
            && old_fingerprint.is_some_and(|fingerprint| {
                fingerprint.windows_recipe_digest == current.windows_recipe_digest
            });
        if !windows_matches {
            clear_windows(owner);
            if previous_windows_evidence {
                invalidated_owners.insert(owner.id.clone());
                event(
                    &mut events,
                    "owner-windows",
                    &owner.id,
                    "reset",
                    "Windows applicability and receipts require a matching verified Windows recipe fingerprint",
                );
            } else {
                event(
                    &mut events,
                    "owner-windows",
                    &owner.id,
                    "pending",
                    "no Windows applicability evidence exists to retain without a verified recipe fingerprint",
                );
            }
        } else {
            event(
                &mut events,
                "owner",
                &owner.id,
                "retained",
                "manifest, tests, and platform recipe fingerprints are unchanged",
            );
        }
    }
    for old_owner in &old.owners {
        if !current_owner_fingerprints.contains_key(&old_owner.id) {
            invalidated_owners.insert(old_owner.id.clone());
            event(
                &mut events,
                "owner",
                &old_owner.id,
                "removed",
                "owner no longer exists in the current recursive source census",
            );
        }
    }

    fresh.schema = CURRENT_SCHEMA;
    fresh.source_digest = source.source_digest.clone();
    fresh.tombstones = tombstones.into_values().collect();
    fresh.publication = old.publication.clone();
    fresh.file_fingerprints = fresh
        .files
        .iter()
        .map(|file| {
            (
                file.path.clone(),
                FileFingerprint {
                    source_digest: file.source_digest.clone(),
                    origin: file.origin.clone(),
                    owner: file.owner.clone(),
                },
            )
        })
        .collect();
    fresh.repository_fingerprints = fresh
        .repositories
        .iter()
        .map(|repository| {
            (
                repository.path.clone(),
                repository_digest(source, &repository.path),
            )
        })
        .collect();
    fresh.owner_fingerprints = current_owner_fingerprints;
    for tombstone in &fresh.tombstones {
        if tombstone.resolution.trim().is_empty() {
            event(
                &mut events,
                "deletion",
                &tombstone.id,
                "pending",
                "deletion has no review resolution",
            );
        }
    }
    gates::check_schema(&fresh, source)?;
    let detected = inventory::scan(root, source)?;
    gates::check_detected_coverage(&fresh, &detected)?;

    let serialized = toml::to_string_pretty(&fresh)?;
    let ledger_digest = hash(serialized.as_bytes());
    let candidates = candidate_report(&detected);
    let mut invalidated_owner_ids: Vec<_> = invalidated_owners.into_iter().collect();
    invalidated_owner_ids.sort();
    let report = ReconciliationReport {
        schema: 1,
        operation: "reconcile".into(),
        ledger_digest,
        source_digest: source.source_digest.clone(),
        previous_source_digest: Some(old.source_digest.clone()),
        previous_schema: Some(previous_schema),
        live_files: fresh.files.len(),
        tombstones: fresh.tombstones.len(),
        events,
        invalidated_owner_ids,
        candidates,
    };
    Ok((fresh, report))
}

pub fn export(root: &Path, source: &Source, ledger: &Ledger) -> Result<HandoffExport> {
    gates::check_schema(ledger, source)?;
    let detected = inventory::scan(root, source)?;
    gates::check_detected_coverage(ledger, &detected)?;
    let inventory = inventory::selections(ledger, Stage::Inventory, false)?;
    let migration = inventory::selections(ledger, Stage::Migration, false)?;
    let windows = inventory::selections(ledger, Stage::Final, true)?;
    let cap_checks: Vec<_> = inventory
        .iter()
        .chain(&migration)
        .chain(&windows)
        .map(|selection| CapCheck {
            selector: selection.selector.clone(),
            files: selection.file_ids.len(),
            occurrences: selection.occurrence_ids.len(),
            deletions: selection.deletion_ids.len(),
            passed: selection.file_ids.len() <= 25
                && selection.occurrence_ids.len() + selection.deletion_ids.len() <= 100,
        })
        .collect();
    if cap_checks.iter().any(|check| !check.passed) {
        return Err("generated handoff exceeds a file or review-item cap".into());
    }
    let ledger_digest = hash(toml::to_string_pretty(ledger)?.as_bytes());
    let selector_digest = hash(serde_json::to_vec(&(
        &ledger_digest,
        &source.source_digest,
        &inventory,
        &migration,
        &windows,
    ))?);
    let pending_files = inventory
        .iter()
        .chain(&migration)
        .chain(&windows)
        .flat_map(|selection| &selection.file_ids)
        .collect::<BTreeSet<_>>()
        .len();
    let pending_occurrences = inventory
        .iter()
        .chain(&migration)
        .chain(&windows)
        .map(|selection| selection.occurrence_ids.len())
        .sum();
    let pending_deletions = inventory
        .iter()
        .map(|selection| selection.deletion_ids.len())
        .sum();
    Ok(HandoffExport {
        schema: 1,
        current_stage: Stage::Inventory,
        ledger_digest,
        source_digest: source.source_digest.clone(),
        selector_digest,
        inventory,
        migration,
        windows,
        pending_files,
        pending_occurrences,
        pending_deletions,
        cap_checks,
    })
}

fn write_json(root: &Path, requested: &Path, value: &impl Serialize) -> Result<()> {
    let path = output_path(root, requested)?;
    fs::write(path, serde_json::to_vec_pretty(value)?)?;
    Ok(())
}

fn companion_path(output: &Path, name: &str) -> Result<PathBuf> {
    let parent = output.parent().ok_or("export output has no parent")?;
    Ok(parent.join(name))
}

pub fn write_reconciliation(
    root: &Path,
    output: &Path,
    ledger: &Ledger,
    report: &ReconciliationReport,
) -> Result<()> {
    let actual_digest = hash(toml::to_string_pretty(ledger)?.as_bytes());
    if actual_digest != report.ledger_digest {
        return Err("reconciliation report does not identify its candidate ledger".into());
    }
    write_json(root, output, report)
}

pub fn write_export(
    root: &Path,
    output: &Path,
    source: &Source,
    ledger: &Ledger,
) -> Result<HandoffExport> {
    let handoffs = export(root, source, ledger)?;
    let candidate = candidate_report(&inventory::scan(root, source)?);
    let report_path = output_path(root, &companion_path(output, "reconciliation.json")?)?;
    let mut report = if report_path.exists() {
        serde_json::from_slice::<ReconciliationReport>(&fs::read(&report_path)?)?
    } else {
        ReconciliationReport {
            schema: 1,
            operation: "export".into(),
            ledger_digest: handoffs.ledger_digest.clone(),
            source_digest: handoffs.source_digest.clone(),
            previous_source_digest: None,
            previous_schema: None,
            live_files: ledger.files.len(),
            tombstones: ledger.tombstones.len(),
            events: vec![],
            invalidated_owner_ids: vec![],
            candidates: candidate.clone(),
        }
    };
    report.ledger_digest = handoffs.ledger_digest.clone();
    report.source_digest = handoffs.source_digest.clone();
    report.candidates = candidate.clone();
    write_json(root, &report_path, &report)?;
    write_json(root, &companion_path(output, "candidates.json")?, &candidate)?;
    write_json(root, output, &handoffs)?;
    write_json(
        root,
        &companion_path(output, "batches-inventory.json")?,
        &handoffs.inventory,
    )?;
    write_json(
        root,
        &companion_path(output, "batches-migration.json")?,
        &handoffs.migration,
    )?;
    write_json(
        root,
        &companion_path(output, "batches-windows.json")?,
        &handoffs.windows,
    )?;
    Ok(handoffs)
}
