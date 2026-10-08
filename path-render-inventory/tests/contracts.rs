use path_render_inventory::{
    gates::{self, PlatformKey},
    hash, inventory,
    model::*,
};
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Fixture {
    root: PathBuf,
    source: Source,
}

impl Fixture {
    fn new(text: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "path-render-contract-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        fs::create_dir(root.join("fixture")).unwrap();
        fs::write(
            root.join("fixture/Cargo.toml"),
            "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[lib]\npath = \"main.rs\"\n",
        )
        .unwrap();
        fs::write(root.join("fixture/main.rs"), text).unwrap();
        let source = Source {
            root_revision: "fixture-revision".into(),
            source_digest: "fixture-source".into(),
            gitlinks: vec![
                format!(" {} fixture", "0".repeat(40)),
                format!(" {} empty-repo", "1".repeat(40)),
            ],
            exclusions: vec![],
            recipe_digests: BTreeMap::from([
                ("linux".into(), "linux-fixture-recipe".into()),
                ("windows".into(), "windows-fixture-recipe".into()),
            ]),
            files: vec![
                SourceFile {
                    path: "fixture/main.rs".into(),
                    hash: hash(text),
                    untracked: true,
                    cargo_packages: vec![],
                },
                SourceFile {
                    path: "fixture/Cargo.toml".into(),
                    hash: hash(fs::read(root.join("fixture/Cargo.toml")).unwrap()),
                    untracked: false,
                    cargo_packages: vec![],
                },
                SourceFile {
                    path: "fixture/Cargo.lock".into(),
                    hash: "lock".into(),
                    untracked: false,
                    cargo_packages: vec![LockPackage {
                        name: "fixture".into(),
                        version: "0.1.0".into(),
                        hash: "selected-lock".into(),
                    }],
                },
            ],
        };
        Self { root, source }
    }

    fn ledger(&self) -> Ledger {
        inventory::scan(&self.root, &self.source).unwrap()
    }

    fn classified(&self) -> Ledger {
        let mut ledger = self.ledger();
        for repo in &mut ledger.repositories {
            repo.reviewed = true;
            repo.rationale = "Fixture repository and generated-source boundary reviewed; zero-Rust repositories included".into();
        }
        for file in &mut ledger.files {
            file.inspected = true;
            file.disposition = if file.occurrences.is_empty() {
                Some(Disposition::NoOccurrence)
            } else {
                None
            };
            file.rationale = "Whole fixture has been inspected".into();
            for gap in &mut file.gaps {
                gap.resolution = "Fixture's complete source was inspected".into();
            }
            for row in &mut file.occurrences {
                row.disposition = Some(Disposition::PreserveFormat);
                row.rationale = "Fixture path is an explicit portable schema identifier".into();
                row.rendering_context =
                    "Portable fixture schema identifier, not a native filesystem display".into();
            }
        }
        ledger
    }

    fn graph(&self) -> CargoGraph {
        CargoGraph {
            workspace_root: self.root.join("fixture").to_str().unwrap().into(),
            packages: vec![CargoPackage {
                id: "fixture-id".into(),
                name: "fixture".into(),
                version: "0.1.0".into(),
                manifest_path: self
                    .root
                    .join("fixture/Cargo.toml")
                    .to_str()
                    .unwrap()
                    .into(),
            }],
            resolve: CargoResolve {
                nodes: vec![CargoNode {
                    id: "fixture-id".into(),
                    dependencies: vec![],
                }],
            },
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}

#[test]
fn batches_include_zero_rust_repository_reviews() {
    let fixture = Fixture::new("fn main() {}\n");
    let mut ledger = fixture.classified();
    let batches = inventory::selections(&ledger, Stage::Inventory, false).unwrap();
    assert!(batches[0].repository_paths.contains(&"empty-repo".into()));
    ledger
        .repositories
        .iter_mut()
        .find(|repo| repo.path == "empty-repo")
        .unwrap()
        .reviewed = false;
    assert!(gates::check_classification(&ledger, &fixture.source, &batches[0]).is_err());
    ledger.files.clear();
    ledger.owners.clear();
    let batches = inventory::selections(&ledger, Stage::Inventory, false).unwrap();
    assert_eq!(batches.len(), 1);
    assert!(batches[0].file_ids.is_empty());
    assert_eq!(batches[0].repository_paths.len(), 3);
}

#[test]
fn classified_occurrences_require_semantic_context() {
    let fixture = Fixture::new("fn f(p: std::path::PathBuf) { p.display().to_string(); }\n");
    let mut ledger = fixture.classified();
    assert_ne!(
        ledger.files[0].occurrences[0].column,
        ledger.files[0].occurrences[1].column
    );
    ledger.files[0].occurrences[0].rendering_context.clear();
    let selection = gates::select(&ledger, "inventory-complete", Stage::Inventory).unwrap();
    assert!(gates::check_classification(&ledger, &fixture.source, &selection).is_err());
}

#[test]
fn consumer_pins_are_checked_against_actual_manifests() {
    let fixture = Fixture::new("fn main() {}\n");
    let mut ledger = fixture.classified();
    let owner = ledger.owners[0].clone();
    let revision = "a".repeat(40);
    ledger.publication = Some(Publication {
        revision: revision.clone(),
        consumer_owners: vec![owner.id.clone()],
    });
    let workspace = fixture.root.join("fixture");
    assert!(gates::check_consumer_pin(&ledger, &owner, &fixture.root, &workspace).is_err());
    let base = fs::read_to_string(workspace.join("Cargo.toml")).unwrap();
    fs::write(
        workspace.join("Cargo.toml"),
        format!("{base}\n[dependencies]\npath-render = {{ path = \"../path-render\" }}\n"),
    )
    .unwrap();
    assert!(gates::check_consumer_pin(&ledger, &owner, &fixture.root, &workspace).is_err());
    fs::write(workspace.join("Cargo.toml"), format!("{base}\n[dependencies]\nrenderer = {{ package = \"path-render\", git = \"https://github.com/mankinskin/workflow-tools\", rev = \"{revision}\" }}\n")).unwrap();
    gates::check_consumer_pin(&ledger, &owner, &fixture.root, &workspace).unwrap();
    fs::write(workspace.join("Cargo.toml"), format!("{base}\n[dependencies]\nrenderer.workspace = true\n[workspace.dependencies]\nrenderer = {{ package = \"path-render\", git = \"https://github.com/mankinskin/workflow-tools\", rev = \"{revision}\" }}\n")).unwrap();
    gates::check_consumer_pin(&ledger, &owner, &fixture.root, &workspace).unwrap();
    ledger.publication.as_mut().unwrap().revision = "b".repeat(40);
    assert!(gates::check_consumer_pin(&ledger, &owner, &fixture.root, &workspace).is_err());
}

#[test]
fn scan_is_deterministic_and_never_classifies_zero_candidates() {
    let fixture = Fixture::new("fn main() {}\n");
    let first = fixture.ledger();
    assert_eq!(
        toml::to_string(&first).unwrap(),
        toml::to_string(&fixture.ledger()).unwrap()
    );
    assert_eq!(first.repositories.len(), 3);
    assert!(first.files[0].occurrences.is_empty());
    assert!(!first.files[0].inspected);
    assert!(!first.files[0].gaps.is_empty());
    assert!(first.files[0].disposition.is_none());
}

#[test]
fn conversions_macros_and_format_implementations_are_candidates() {
    let fixture = Fixture::new(
        "fn f(p: std::path::PathBuf) { println!(\"{:?}\", p); p.display().to_string(); p.to_string_lossy(); }\nimpl std::fmt::Debug for X { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, \"x\") } }\n",
    );
    let ledger = fixture.ledger();
    assert!(ledger.files[0].occurrences.len() >= 6);
    assert!(
        ledger.files[0]
            .occurrences
            .iter()
            .all(|row| row.disposition.is_none())
    );
}

#[test]
fn broken_rust_and_excluded_generated_sources_remain_explicit_gaps() {
    let mut fixture = Fixture::new("fn broken(");
    fixture.source.exclusions.push(Exclusion {
        path: "fixture/target/generated.rs".into(),
        reason: "source-allowlist".into(),
    });
    let ledger = fixture.ledger();
    assert_eq!(ledger.files.len(), 2);
    assert!(
        ledger
            .files
            .iter()
            .flat_map(|file| &file.gaps)
            .any(|gap| gap.reason.contains("parsing failed"))
    );
    assert!(
        ledger
            .files
            .iter()
            .any(|file| file.origin == "source-allowlist" && !file.inspected)
    );
}

#[test]
fn batches_respect_both_ceilings_and_split_large_files() {
    let fixture = Fixture::new("fn f(p: std::path::PathBuf) { p.display(); }\n");
    let mut ledger = fixture.ledger();
    let template = ledger.files[0].occurrences[0].clone();
    ledger.files[0].occurrences = (0..205)
        .map(|number| Occurrence {
            id: format!("row-{number:03}"),
            line: number + 1,
            ..template.clone()
        })
        .collect();
    let selections = inventory::selections(&ledger, Stage::Inventory, false).unwrap();
    assert_eq!(selections.len(), 3);
    assert_eq!(selections[0].occurrence_ids.len(), 100);
    assert_eq!(selections[2].occurrence_ids.len(), 5);
    ledger.files[0].occurrences.clear();
    let template = ledger.files[0].clone();
    ledger.files = (0..52)
        .map(|number| FileReview {
            id: format!("file-{number:03}"),
            path: format!("fixture/{number:03}.rs"),
            ..template.clone()
        })
        .collect();
    let selections = inventory::selections(&ledger, Stage::Inventory, false).unwrap();
    assert_eq!(selections.len(), 3);
    assert!(
        selections.iter().all(
            |selection| selection.file_ids.len() <= 25 && selection.occurrence_ids.len() <= 100
        )
    );
    assert_eq!(selections[0].selector, "batch:remaining:001");
}

#[test]
fn duplicate_unowned_and_omitted_rows_fail() {
    let fixture = Fixture::new("fn f(p: std::path::PathBuf) { p.display(); }\n");
    let mut ledger = fixture.classified();
    gates::check_schema(&ledger, &fixture.source).unwrap();
    ledger.files.push(ledger.files[0].clone());
    assert!(gates::check_schema(&ledger, &fixture.source).is_err());
    ledger.files.pop();
    ledger.files[0].owner = Some("unowned".into());
    assert!(gates::check_schema(&ledger, &fixture.source).is_err());
    ledger.files[0].owner = Some(ledger.owners[0].id.clone());
    ledger.files[0].occurrences.clear();
    assert!(gates::check_detected_coverage(&ledger, &fixture.ledger()).is_err());
}

#[test]
fn unresolved_coverage_and_exemptions_without_rationale_fail() {
    let fixture = Fixture::new("fn main() {}\n");
    let mut ledger = fixture.classified();
    let selection = gates::select(&ledger, "inventory-complete", Stage::Inventory).unwrap();
    gates::check_classification(&ledger, &fixture.source, &selection).unwrap();
    ledger.files[0].gaps[0].resolution.clear();
    assert!(gates::check_classification(&ledger, &fixture.source, &selection).is_err());
    ledger.files[0].gaps[0].resolution = "Reviewed".into();
    ledger.files[0].disposition = Some(Disposition::Exception);
    ledger.files[0].rationale.clear();
    assert!(gates::check_classification(&ledger, &fixture.source, &selection).is_err());
}

#[test]
fn inventory_completion_is_not_migration_completion() {
    let fixture = Fixture::new("fn f(p: std::path::PathBuf) { p.display(); }\n");
    let mut ledger = fixture.classified();
    ledger.files[0].occurrences[0].disposition = Some(Disposition::Migrate);
    let inventory = gates::select(&ledger, "inventory-complete", Stage::Inventory).unwrap();
    gates::check_classification(&ledger, &fixture.source, &inventory).unwrap();
    let migration = gates::select(&ledger, "linux-migration-complete", Stage::Migration).unwrap();
    assert!(gates::check_classification(&ledger, &fixture.source, &migration).is_err());
    assert!(gates::select(&ledger, "inventory-complete", Stage::Final).is_err());
    assert!(gates::select(&ledger, "unknown", Stage::Inventory).is_err());
}

#[test]
fn changed_source_and_missing_zero_rust_repository_review_fail() {
    let mut fixture = Fixture::new("fn main() {}\n");
    let mut ledger = fixture.classified();
    let selection = gates::select(&ledger, "inventory-complete", Stage::Inventory).unwrap();
    fixture.source.files[0].hash = "changed".into();
    assert!(gates::check_classification(&ledger, &fixture.source, &selection).is_err());
    fixture.source.files[0].hash = ledger.files[0].source_digest.clone();
    ledger
        .repositories
        .iter_mut()
        .find(|repo| repo.path == "empty-repo")
        .unwrap()
        .reviewed = false;
    assert!(gates::check_classification(&ledger, &fixture.source, &selection).is_err());
}

#[test]
fn owner_digest_ignores_unrelated_files_but_rejects_dependency_lock_changes() {
    let mut fixture = Fixture::new("fn main() {}\n");
    let ledger = fixture.classified();
    let owner = &ledger.owners[0];
    let graph = fixture.graph();
    let before = gates::owner_digest(
        owner,
        &graph,
        &fixture.source,
        &fixture.root,
        Platform::Linux,
    )
    .unwrap();
    fixture.source.files.push(SourceFile {
        path: "unrelated/lib.rs".into(),
        hash: "unrelated-change".into(),
        untracked: true,
        cargo_packages: vec![],
    });
    assert_eq!(
        before,
        gates::owner_digest(
            owner,
            &graph,
            &fixture.source,
            &fixture.root,
            Platform::Linux
        )
        .unwrap()
    );
    fixture
        .source
        .recipe_digests
        .insert("linux".into(), "changed-base-recipe".into());
    assert_ne!(
        before,
        gates::owner_digest(
            owner,
            &graph,
            &fixture.source,
            &fixture.root,
            Platform::Linux
        )
        .unwrap()
    );
    fixture
        .source
        .recipe_digests
        .insert("linux".into(), "linux-fixture-recipe".into());
    fixture.source.files[2].cargo_packages[0].hash = "dependency-change".into();
    assert_ne!(
        before,
        gates::owner_digest(
            owner,
            &graph,
            &fixture.source,
            &fixture.root,
            Platform::Linux
        )
        .unwrap()
    );
}

#[test]
fn stale_owner_receipts_and_unjustified_windows_exemptions_fail() {
    let fixture = Fixture::new("fn main() {}\n");
    let mut ledger = fixture.classified();
    let owner = &ledger.owners[0];
    let selection = gates::select(&ledger, "linux-migration-complete", Stage::Migration).unwrap();
    let current = BTreeMap::from([(
        (owner.id.clone(), PlatformKey::Linux),
        "current-owner".into(),
    )]);
    let mut proof = OwnerProof {
        owner: owner.id.clone(),
        platform: Platform::Linux,
        content_digest: "stale".into(),
        test_args: owner.test_args.clone(),
        recipe: owner.linux_recipe.clone(),
        passed: true,
    };
    assert!(gates::check_evidence(&ledger, &selection, &current, &[proof.clone()]).is_err());
    ledger.owners[0].windows = WindowsApplicability::Inapplicable;
    ledger.owners[0].windows_rationale =
        "This fixture has no Windows build surface; verified fixture-only policy".into();
    assert!(gates::check_evidence(&ledger, &selection, &current, &[proof.clone()]).is_err());
    proof.content_digest = "current-owner".into();
    gates::check_evidence(&ledger, &selection, &current, &[proof]).unwrap();
    ledger.owners[0].windows_rationale.clear();
    assert!(gates::check_evidence(&ledger, &selection, &current, &[]).is_err());
}

#[test]
fn final_gate_requires_both_platforms_and_published_consumer_pin() {
    let fixture = Fixture::new("fn f(p: std::path::PathBuf) { p.display(); }\n");
    let mut ledger = fixture.classified();
    ledger.files[0].occurrences[0].disposition = Some(Disposition::Migrate);
    ledger.files[0].occurrences[0].implemented = true;
    let owner = &mut ledger.owners[0];
    owner.windows = WindowsApplicability::Applicable;
    owner.windows_test_args = owner.test_args.clone();
    owner.windows_recipe = "rust-servercore-ltsc2022-msvc".into();
    let id = owner.id.clone();
    let linux = OwnerProof {
        owner: id.clone(),
        platform: Platform::Linux,
        content_digest: "linux-current".into(),
        test_args: owner.test_args.clone(),
        recipe: owner.linux_recipe.clone(),
        passed: true,
    };
    let windows = OwnerProof {
        owner: id.clone(),
        platform: Platform::Windows,
        content_digest: "windows-current".into(),
        test_args: owner.windows_test_args.clone(),
        recipe: owner.windows_recipe.clone(),
        passed: true,
    };
    let current = BTreeMap::from([
        ((id.clone(), PlatformKey::Linux), "linux-current".into()),
        ((id.clone(), PlatformKey::Windows), "windows-current".into()),
    ]);
    let selection = gates::select(&ledger, "final-both-platform-complete", Stage::Final).unwrap();
    assert!(gates::check_evidence(&ledger, &selection, &current, &[linux.clone()]).is_err());
    ledger.publication = Some(Publication {
        revision: "a".repeat(40),
        consumer_owners: vec![id],
    });
    assert!(
        gates::check_evidence(
            &ledger,
            &selection,
            &current,
            &[linux.clone(), windows.clone()]
        )
        .is_err()
    );
    ledger.owners[0].dependency_revision = Some("a".repeat(40));
    gates::check_evidence(&ledger, &selection, &current, &[linux, windows]).unwrap();
}

#[test]
fn native_fixture_source_digest_and_toml_roundtrip_match() {
    let fixture = Fixture::new("fn main() {}\n");
    let encoded = toml::to_string_pretty(&fixture.classified()).unwrap();
    let decoded: Ledger = toml::from_str(&encoded).unwrap();
    gates::check_schema(&decoded, &fixture.source).unwrap();
    gates::check_detected_coverage(&decoded, &fixture.ledger()).unwrap();
}

#[test]
fn cli_generation_refuses_to_overwrite_and_emits_exact_batches() {
    let fixture = Fixture::new("fn main() {}\n");
    let source = fixture.root.join("source.json");
    let output = fixture.root.join("ledger.toml");
    fs::write(&source, serde_json::to_vec(&fixture.source).unwrap()).unwrap();
    let invoke = || {
        std::process::Command::new(env!("CARGO_BIN_EXE_path-render-inventory"))
            .arg("scan")
            .arg("--root")
            .arg(&fixture.root)
            .arg("--source")
            .arg(&source)
            .arg("--output")
            .arg(&output)
            .output()
            .unwrap()
    };
    assert!(invoke().status.success());
    assert!(!invoke().status.success());
    let response = std::process::Command::new(env!("CARGO_BIN_EXE_path-render-inventory"))
        .arg("batches")
        .arg("--ledger")
        .arg(&output)
        .args(["--stage", "inventory"])
        .output()
        .unwrap();
    assert!(response.status.success());
    let selectors: Vec<Selection> = serde_json::from_slice(&response.stdout).unwrap();
    assert_eq!(selectors[0].selector, "batch:remaining:001");
}

#[test]
fn cli_inventory_gate_passes_but_rejects_unresolved_whole_file_review() {
    let fixture = Fixture::new("fn main() {}\n");
    let source = fixture.root.join("source.json");
    let ledger_path = fixture.root.join("ledger.toml");
    let output = fixture.root.join("selection.json");
    let receipts = fixture.root.join("receipts");
    fs::create_dir(&receipts).unwrap();
    fs::write(&source, serde_json::to_vec(&fixture.source).unwrap()).unwrap();
    let invoke = || {
        std::process::Command::new(env!("CARGO_BIN_EXE_path-render-inventory"))
            .env("WORKFLOW_VALIDATION_CONTAINER", "platform")
            .arg("validate")
            .arg("--root")
            .arg(&fixture.root)
            .arg("--source")
            .arg(&source)
            .arg("--ledger")
            .arg(&ledger_path)
            .args([
                "--selector",
                "inventory-complete",
                "--stage",
                "inventory",
                "--platform",
                "linux",
            ])
            .arg("--receipts")
            .arg(&receipts)
            .arg("--output")
            .arg(&output)
            .arg("--verify-complete")
            .output()
            .unwrap()
    };
    fs::write(
        &ledger_path,
        toml::to_string(&fixture.classified()).unwrap(),
    )
    .unwrap();
    let passed = invoke();
    assert!(
        passed.status.success(),
        "{}",
        String::from_utf8_lossy(&passed.stderr)
    );
    let result: ValidationResult = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
    assert!(result.selection.aggregate);
    assert!(result.owners.is_empty());
    fs::write(&ledger_path, toml::to_string(&fixture.ledger()).unwrap()).unwrap();
    assert!(!invoke().status.success());
}

#[test]
fn cli_runs_concrete_native_owner_and_exports_its_proof() {
    let mut fixture = Fixture::new("pub fn fixture() {}\n");
    let generated = std::process::Command::new("cargo")
        .current_dir(&fixture.root)
        .args([
            "generate-lockfile",
            "--offline",
            "--manifest-path",
            "fixture/Cargo.toml",
        ])
        .output()
        .unwrap();
    assert!(
        generated.status.success(),
        "{}",
        String::from_utf8_lossy(&generated.stderr)
    );
    let lock = fs::read(fixture.root.join("fixture/Cargo.lock")).unwrap();
    fixture.source.files[2].hash = hash(&lock);
    let mut ledger = fixture.classified();
    let platform = if cfg!(windows) {
        Platform::Windows
    } else {
        Platform::Linux
    };
    if platform == Platform::Windows {
        ledger.owners[0].windows = WindowsApplicability::Applicable;
        ledger.owners[0].windows_test_args = ledger.owners[0].test_args.clone();
        ledger.owners[0].windows_recipe = "rust-servercore-ltsc2022-msvc".into();
    }
    let source = fixture.root.join("source.json");
    let ledger_path = fixture.root.join("ledger.toml");
    let receipts = fixture.root.join("receipts");
    let output = fixture.root.join("selection.json");
    fs::create_dir(&receipts).unwrap();
    fs::write(&source, serde_json::to_vec(&fixture.source).unwrap()).unwrap();
    fs::write(&ledger_path, toml::to_string(&ledger).unwrap()).unwrap();
    let response = std::process::Command::new(env!("CARGO_BIN_EXE_path-render-inventory"))
        .env("WORKFLOW_VALIDATION_CONTAINER", "platform")
        .args([
            "validate",
            "--selector",
            "batch:remaining:001",
            "--stage",
            "inventory",
            "--platform",
            if cfg!(windows) { "windows" } else { "linux" },
            "--run-selected-tests",
            "--check-ledger",
        ])
        .arg("--root")
        .arg(&fixture.root)
        .arg("--source")
        .arg(&source)
        .arg("--ledger")
        .arg(&ledger_path)
        .arg("--receipts")
        .arg(&receipts)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        response.status.success(),
        "{}",
        String::from_utf8_lossy(&response.stderr)
    );
    let result: ValidationResult = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
    assert_eq!(result.owners.len(), 1);
    assert!(result.owners[0].proof.passed);
    assert_eq!(result.owners[0].proof.platform, platform);
    assert!(fixture.root.join(&result.owners[0].metadata_file).exists());
}

#[test]
fn cli_cannot_generate_foreign_platform_evidence() {
    let fixture = Fixture::new("fn main() {}\n");
    let foreign = if cfg!(windows) { "linux" } else { "windows" };
    let result = std::process::Command::new(env!("CARGO_BIN_EXE_path-render-inventory"))
        .env("WORKFLOW_VALIDATION_CONTAINER", "platform")
        .args([
            "validate",
            "--selector",
            "batch:remaining:001",
            "--stage",
            "inventory",
            "--platform",
            foreign,
        ])
        .arg("--root")
        .arg(&fixture.root)
        .arg("--source")
        .arg("unused")
        .arg("--ledger")
        .arg("unused")
        .arg("--receipts")
        .arg("unused")
        .arg("--output")
        .arg("unused")
        .arg("--run-selected-tests")
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("native container OS"));
}

#[test]
fn ambiguous_owner_arguments_and_workspace_escape_are_rejected() {
    let fixture = Fixture::new("fn main() {}\n");
    let mut ledger = fixture.ledger();
    ledger.owners[0].test_args.push("--workspace".into());
    assert!(gates::test_arguments(&ledger.owners[0], Platform::Linux).is_err());
    assert!(path_render_inventory::input_path(&fixture.root, "../outside.rs").is_err());
    assert!(path_render_inventory::input_path(&fixture.root, "fixture\\main.rs").is_err());
}

#[cfg(unix)]
#[test]
fn linked_sources_are_not_followed() {
    let fixture = Fixture::new("fn main() {}\n");
    std::os::unix::fs::symlink("main.rs", fixture.root.join("fixture/linked.rs")).unwrap();
    assert!(path_render_inventory::input_path(&fixture.root, "fixture/linked.rs").is_err());
}

#[test]
fn windows_batch_does_not_require_its_future_paired_linux_run() {
    let fixture = Fixture::new("fn main() {}\n");
    let mut ledger = fixture.classified();
    let owner = &mut ledger.owners[0];
    owner.windows = WindowsApplicability::Applicable;
    owner.windows_test_args = owner.test_args.clone();
    owner.windows_recipe = "rust-servercore-ltsc2022-msvc".into();
    let id = owner.id.clone();
    let proof = OwnerProof {
        owner: id.clone(),
        platform: Platform::Windows,
        content_digest: "windows-current".into(),
        test_args: owner.windows_test_args.clone(),
        recipe: owner.windows_recipe.clone(),
        passed: true,
    };
    ledger.publication = Some(Publication {
        revision: "a".repeat(40),
        consumer_owners: vec![],
    });
    let current = BTreeMap::from([((id, PlatformKey::Windows), "windows-current".into())]);
    let selection = gates::select(&ledger, "windows-batch:remaining:001", Stage::Final).unwrap();
    gates::check_platform_evidence(&ledger, &selection, &current, &[proof], Platform::Windows)
        .unwrap();
    let final_gate = gates::select(&ledger, "final-both-platform-complete", Stage::Final).unwrap();
    assert!(gates::check_evidence(&ledger, &final_gate, &current, &[]).is_err());
}
