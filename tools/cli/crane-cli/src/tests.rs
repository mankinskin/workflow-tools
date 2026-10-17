use std::{
    fs,
    io::Cursor,
    path::{
        Path,
        PathBuf,
    },
    process::Command,
};

use tempfile::TempDir;

use crate::{
    CraneCli,
    CraneCommand,
    CraneError,
    TransplantArgs,
    cli::{
        parse_mapping,
        validate_mappings,
    },
    execute,
    run,
    transform::{
        remap_path,
        transform_export,
    },
};

#[test]
fn transplant_imports_filtered_history_into_target_repo() {
    let temp = TempDir::new().expect("tempdir should exist");
    let source_repo = temp.path().join("source");
    let target_repo = temp.path().join("target");

    init_repo(&source_repo);
    commit_file(&source_repo, "README.md", "root\n", "initial root commit");
    commit_files(
        &source_repo,
        &[
            ("tools/cli/context-cli/src/main.rs", "fn main() {}\n"),
            ("tools/http/context-http/src/lib.rs", "pub fn serve() {}\n"),
            ("tools/mcp/context-mcp/src/main.rs", "fn main() {}\n"),
        ],
        "shared tools scaffold",
    );
    commit_file(
        &source_repo,
        "tools/context-editor/kernel/Cargo.toml",
        "[package]\nname = \"context-editor-kernel\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        "context editor init",
    );
    commit_file(
        &source_repo,
        "unrelated/file.txt",
        "ignore me\n",
        "unrelated change",
    );
    commit_files(
        &source_repo,
        &[
            (
                "tools/cli/context-cli/src/main.rs",
                "fn main() { println!(\"cli\"); }\n",
            ),
            (
                "tools/http/context-http/src/lib.rs",
                "pub fn serve() { println!(\"http\"); }\n",
            ),
        ],
        "shared cli http update",
    );

    init_repo(&target_repo);
    commit_file(
        &target_repo,
        "Cargo.toml",
        "[workspace]\nresolver = \"2\"\nmembers = []\n",
        "target init",
    );

    let args = TransplantArgs {
        source_repo: source_repo.clone(),
        target_repo: target_repo.clone(),
        source_ref: "HEAD".to_string(),
        target_branch: "main".to_string(),
        import_branch: "crane/import".to_string(),
        anchor_commit: None,
        mappings: vec![
            parse_mapping("tools/cli/context-cli=tools/cli/context-cli")
                .expect("cli mapping should parse"),
            parse_mapping("tools/mcp/context-mcp=tools/mcp/context-mcp")
                .expect("mcp mapping should parse"),
            parse_mapping("tools/http/context-http=tools/http/context-http")
                .expect("http mapping should parse"),
            parse_mapping("tools/context-editor=tools/context-editor")
                .expect("editor mapping should parse"),
        ],
        no_merge: false,
        dry_run: false,
    };

    let outcome = execute(args).expect("transplant should succeed");

    assert!(outcome.merged);
    assert!(outcome.stats.is_some());
    assert_eq!(
        fs::read_to_string(
            target_repo.join("tools/cli/context-cli/src/main.rs")
        )
        .expect("cli file should exist"),
        "fn main() { println!(\"cli\"); }\n"
    );
    assert!(
        target_repo
            .join("tools/context-editor/kernel/Cargo.toml")
            .exists(),
        "context-editor history should be imported"
    );

    let combined_log = git_output(
        &target_repo,
        &[
            "log",
            "--format=%s",
            "--all",
            "--",
            "tools/cli/context-cli",
            "tools/http/context-http",
            "tools/mcp/context-mcp",
            "tools/context-editor",
        ],
    );
    assert!(combined_log.contains("shared tools scaffold"));
    assert!(combined_log.contains("shared cli http update"));
    assert!(combined_log.contains("context editor init"));
    assert!(
        !combined_log.contains("unrelated change"),
        "unrelated history should be excluded"
    );
}

#[test]
fn parse_mapping_allows_empty_destination_for_branch_root() {
    assert_eq!(
        parse_mapping("crates/context-stack=")
            .expect("branch-root mapping should parse"),
        crate::PathMapping {
            source: "crates/context-stack".to_string(),
            destination: String::new(),
        }
    );
}

#[test]
fn validate_mappings_rejects_overlapping_destinations() {
    let error = validate_mappings(&[
        parse_mapping("crates/context-stack=")
            .expect("branch-root mapping should parse"),
        parse_mapping("tools/context-editor=tools/context-editor")
            .expect("same-path mapping should parse"),
    ])
    .expect_err("overlapping destination scopes must fail");

    assert!(matches!(error, CraneError::BadRequest(_)));
    assert!(
        error
            .to_string()
            .contains("overlapping destination mappings are not supported")
    );
}

#[test]
fn transform_export_rewrites_branch_root_file_ops() {
    let mapping = parse_mapping("crates/context-stack=")
        .expect("branch-root mapping should parse");
    assert_eq!(
        remap_path("crates/context-stack/Cargo.toml", &[mapping.clone()]),
        Some("Cargo.toml".to_string())
    );

    let mut output = Vec::new();
    let stats = transform_export(
        Cursor::new(concat!(
            "commit refs/heads/source\n",
            "mark :1\n",
            "author Crane Test <crane@example.com> 0 +0000\n",
            "committer Crane Test <crane@example.com> 0 +0000\n",
            "data 5\n",
            "test\n",
            "M 100644 :blob crates/context-stack/Cargo.toml\n",
            "M 100644 :blob crates/context-stack/src/lib.rs\n",
            "R crates/context-stack/src/lib.rs crates/context-stack/src/main.rs\n",
            "C crates/context-stack/src/main.rs crates/context-stack/examples/demo.rs\n",
            "D crates/context-stack/src/old.rs\n",
            "\n"
        )),
        &mut output,
        "refs/heads/crane/root",
        &[mapping],
    )
    .expect("branch-root transform should succeed");

    let output =
        String::from_utf8(output).expect("transform output should be utf8");

    assert_eq!(stats.commit_count, 1);
    assert_eq!(stats.rewritten_ops, 5);
    assert!(output.contains("commit refs/heads/crane/root\n"));
    assert!(output.contains("M 100644 :blob Cargo.toml\n"));
    assert!(output.contains("M 100644 :blob src/lib.rs\n"));
    assert!(output.contains("R src/lib.rs src/main.rs\n"));
    assert!(output.contains("C src/main.rs examples/demo.rs\n"));
    assert!(output.contains("D src/old.rs\n"));
}

#[test]
fn transplant_same_path_context_tools_excludes_sibling_prefixes() {
    let fixture = setup_same_path_context_tools_fixture();

    let outcome = execute(TransplantArgs {
        source_repo: fixture.source_repo.clone(),
        target_repo: fixture.target_repo.clone(),
        source_ref: "HEAD".to_string(),
        target_branch: "main".to_string(),
        import_branch: "crane/context-tools".to_string(),
        anchor_commit: None,
        mappings: tool_mappings(),
        no_merge: false,
        dry_run: false,
    })
    .expect("same-path transplant should succeed");

    assert!(outcome.merged);
    assert_eq!(outcome.plan.anchor_commit, fixture.anchor_commit);
    assert_eq!(outcome.plan.source_commit, fixture.head_commit);

    assert_eq!(
        fs::read_to_string(
            fixture
                .target_repo
                .join("tools/cli/context-cli/src/main.rs")
        )
        .expect("selected cli file should exist"),
        "fn main() { println!(\"cli\"); }\n"
    );
    assert_eq!(
        fs::read_to_string(
            fixture
                .target_repo
                .join("tools/http/context-http/src/lib.rs")
        )
        .expect("selected http file should exist"),
        "pub fn serve() { println!(\"http\"); }\n"
    );
    assert!(
        !fixture
            .target_repo
            .join("tools/http/context-http/src/router.rs")
            .exists(),
        "deletes inside selected paths should carry across"
    );

    for excluded_path in [
        "tools/cli/context-cli-extra/src/main.rs",
        "tools/http/context-http-extra/src/lib.rs",
        "tools/mcp/context-mcp-extra/src/main.rs",
        "tools/context-editor-old/kernel/Cargo.toml",
    ] {
        assert!(
            !fixture.target_repo.join(excluded_path).exists(),
            "excluded sibling path should not be imported: {excluded_path}"
        );
    }

    let selected_log = git_output(
        &fixture.target_repo,
        &[
            "log",
            "--format=%s",
            "--all",
            "--",
            "tools/cli/context-cli",
            "tools/http/context-http",
            "tools/mcp/context-mcp",
            "tools/context-editor",
        ],
    );
    assert!(selected_log.contains("tool family scaffold"));
    assert!(selected_log.contains("mixed selected and sibling updates"));
    assert!(
        !selected_log.contains("excluded sibling only update"),
        "commits touching only excluded siblings should not be imported"
    );

    let excluded_log = git_output(
        &fixture.target_repo,
        &[
            "log",
            "--format=%s",
            "--all",
            "--",
            "tools/cli/context-cli-extra",
            "tools/http/context-http-extra",
            "tools/mcp/context-mcp-extra",
            "tools/context-editor-old",
        ],
    );
    assert!(
        excluded_log.trim().is_empty(),
        "excluded siblings should not carry history into the target"
    );
}

#[test]
fn dry_run_reports_reviewable_plan_metadata() {
    let temp = TempDir::new().expect("tempdir should exist");
    let source_repo = temp.path().join("source");
    let target_repo = temp.path().join("target");

    init_repo(&source_repo);
    let initial_commit =
        commit_file(&source_repo, "README.md", "root\n", "initial root commit");
    let first_tool_commit = commit_files(
        &source_repo,
        &[
            ("tools/cli/context-cli/src/main.rs", "fn main() {}\n"),
            ("tools/http/context-http/src/lib.rs", "pub fn serve() {}\n"),
            ("tools/mcp/context-mcp/src/main.rs", "fn main() {}\n"),
        ],
        "shared tools scaffold",
    );
    commit_file(
        &source_repo,
        "tools/context-editor/kernel/Cargo.toml",
        "[package]\nname = \"context-editor-kernel\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        "context editor init",
    );
    commit_file(
        &source_repo,
        "unrelated/file.txt",
        "ignore me\n",
        "unrelated change",
    );
    let head_commit = commit_files(
        &source_repo,
        &[
            (
                "tools/cli/context-cli/src/main.rs",
                "fn main() { println!(\"cli\"); }\n",
            ),
            (
                "tools/http/context-http/src/lib.rs",
                "pub fn serve() { println!(\"http\"); }\n",
            ),
        ],
        "shared cli http update",
    );

    init_repo(&target_repo);
    commit_file(
        &target_repo,
        "Cargo.toml",
        "[workspace]\nresolver = \"2\"\nmembers = []\n",
        "target init",
    );

    let args = TransplantArgs {
        source_repo: source_repo.clone(),
        target_repo: target_repo.clone(),
        source_ref: "HEAD".to_string(),
        target_branch: "main".to_string(),
        import_branch: "crane/tools-review".to_string(),
        anchor_commit: None,
        mappings: tool_mappings(),
        no_merge: false,
        dry_run: true,
    };
    let expected_mappings =
        validate_mappings(&tool_mappings()).expect("mappings should normalize");

    let outcome = execute(args.clone()).expect("dry-run should succeed");
    assert!(!outcome.merged, "dry-run must not report a merge");
    assert!(
        outcome.stats.is_none(),
        "dry-run must not emit import stats"
    );
    assert_eq!(outcome.plan.source_ref, "HEAD");
    assert_eq!(outcome.plan.source_commit, head_commit);
    assert_eq!(outcome.plan.anchor_commit, first_tool_commit);
    assert_eq!(
        outcome.plan.range_spec,
        format!("{initial_commit}..{head_commit}")
    );
    assert_eq!(outcome.plan.target_branch, "main");
    assert_eq!(outcome.plan.import_branch, "crane/tools-review");
    assert_eq!(outcome.plan.import_ref, "refs/heads/crane/tools-review");
    assert_eq!(outcome.plan.mappings, expected_mappings);

    let output = run(CraneCli {
        command: CraneCommand::Transplant(args),
    })
    .expect("dry-run output should render");

    assert!(output.contains("source_ref=HEAD"));
    assert!(output.contains(&format!("source_commit={head_commit}")));
    assert!(output.contains(&format!("anchor_commit={first_tool_commit}")));
    assert!(
        output.contains(&format!("range_spec={initial_commit}..{head_commit}"))
    );
    assert!(output.contains("target_branch=main"));
    assert!(output.contains("import_branch=crane/tools-review"));
    assert!(output.contains("import_ref=refs/heads/crane/tools-review"));
    assert!(
        output.contains("mapping=tools/context-editor=tools/context-editor")
    );
    assert!(output.contains("dry_run=true"));
}

#[test]
fn dry_run_leaves_same_path_target_repo_untouched() {
    let fixture = setup_same_path_context_tools_fixture();
    let target_head_before =
        git_output(&fixture.target_repo, &["rev-parse", "HEAD"])
            .trim()
            .to_string();

    assert!(
        !git_ref_exists(
            &fixture.target_repo,
            "refs/heads/crane/context-tools-review"
        ),
        "fixture target should not start with the import ref"
    );

    let outcome = execute(TransplantArgs {
        source_repo: fixture.source_repo.clone(),
        target_repo: fixture.target_repo.clone(),
        source_ref: "HEAD".to_string(),
        target_branch: "main".to_string(),
        import_branch: "crane/context-tools-review".to_string(),
        anchor_commit: None,
        mappings: tool_mappings(),
        no_merge: false,
        dry_run: true,
    })
    .expect("same-path dry-run should succeed");

    assert!(!outcome.merged);
    assert!(outcome.stats.is_none());
    assert_eq!(outcome.plan.anchor_commit, fixture.anchor_commit);
    assert_eq!(outcome.plan.source_commit, fixture.head_commit);
    assert_eq!(
        outcome.plan.range_spec,
        format!("{}..{}", fixture.initial_commit, fixture.head_commit)
    );

    let target_head_after =
        git_output(&fixture.target_repo, &["rev-parse", "HEAD"])
            .trim()
            .to_string();
    assert_eq!(target_head_after, target_head_before);
    assert!(
        git_output(&fixture.target_repo, &["status", "--short"])
            .trim()
            .is_empty(),
        "dry-run should not dirty the target repo"
    );
    assert!(
        !git_ref_exists(
            &fixture.target_repo,
            "refs/heads/crane/context-tools-review"
        ),
        "dry-run should not create the import ref"
    );
}

fn tool_mappings() -> Vec<crate::PathMapping> {
    vec![
        parse_mapping("tools/cli/context-cli=tools/cli/context-cli")
            .expect("cli mapping should parse"),
        parse_mapping("tools/mcp/context-mcp=tools/mcp/context-mcp")
            .expect("mcp mapping should parse"),
        parse_mapping("tools/http/context-http=tools/http/context-http")
            .expect("http mapping should parse"),
        parse_mapping("tools/context-editor=tools/context-editor")
            .expect("editor mapping should parse"),
    ]
}

struct SamePathContextToolsFixture {
    _temp: TempDir,
    source_repo: PathBuf,
    target_repo: PathBuf,
    initial_commit: String,
    anchor_commit: String,
    head_commit: String,
}

fn setup_same_path_context_tools_fixture() -> SamePathContextToolsFixture {
    let temp = TempDir::new().expect("tempdir should exist");
    let source_repo = temp.path().join("source");
    let target_repo = temp.path().join("target");

    init_repo(&source_repo);
    let initial_commit =
        commit_file(&source_repo, "README.md", "root\n", "initial root commit");
    let anchor_commit = commit_files(
        &source_repo,
        &[
            ("tools/cli/context-cli/src/main.rs", "fn main() {}\n"),
            (
                "tools/cli/context-cli/src/output.rs",
                "pub fn output() {}\n",
            ),
            ("tools/http/context-http/src/lib.rs", "pub fn serve() {}\n"),
            (
                "tools/http/context-http/src/router.rs",
                "pub fn route() {}\n",
            ),
            ("tools/mcp/context-mcp/src/main.rs", "fn main() {}\n"),
            (
                "tools/context-editor/kernel/Cargo.toml",
                "[package]\nname = \"context-editor-kernel\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
            ),
            (
                "tools/cli/context-cli-extra/src/main.rs",
                "fn main() { println!(\"cli-extra\"); }\n",
            ),
            (
                "tools/http/context-http-extra/src/lib.rs",
                "pub fn serve_extra() {}\n",
            ),
            (
                "tools/mcp/context-mcp-extra/src/main.rs",
                "fn main() { println!(\"mcp-extra\"); }\n",
            ),
            (
                "tools/context-editor-old/kernel/Cargo.toml",
                "[package]\nname = \"context-editor-old\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
            ),
        ],
        "tool family scaffold",
    );
    commit_file(
        &source_repo,
        "tools/http/context-http-extra/README.md",
        "excluded sibling only\n",
        "excluded sibling only update",
    );
    let _ = fs::remove_file(
        source_repo.join("tools/http/context-http/src/router.rs"),
    );
    let head_commit = commit_files(
        &source_repo,
        &[
            (
                "tools/cli/context-cli/src/main.rs",
                "fn main() { println!(\"cli\"); }\n",
            ),
            (
                "tools/http/context-http/src/lib.rs",
                "pub fn serve() { println!(\"http\"); }\n",
            ),
            (
                "tools/cli/context-cli-extra/src/main.rs",
                "fn main() { println!(\"cli-extra-updated\"); }\n",
            ),
            (
                "tools/http/context-http-extra/src/lib.rs",
                "pub fn serve_extra() { println!(\"extra\"); }\n",
            ),
        ],
        "mixed selected and sibling updates",
    );

    init_repo(&target_repo);
    commit_file(
        &target_repo,
        "Cargo.toml",
        "[workspace]\nresolver = \"2\"\nmembers = []\n",
        "target init",
    );

    SamePathContextToolsFixture {
        _temp: temp,
        source_repo,
        target_repo,
        initial_commit,
        anchor_commit,
        head_commit,
    }
}

fn init_repo(path: &Path) {
    fs::create_dir_all(path).expect("repo dir should exist");
    run_git(path, &["init", "-b", "main"]);
    run_git(path, &["config", "user.name", "Crane Test"]);
    run_git(path, &["config", "user.email", "crane@example.com"]);
}

fn commit_file(
    repo: &Path,
    file_path: &str,
    contents: &str,
    message: &str,
) -> String {
    commit_files(repo, &[(file_path, contents)], message)
}

fn commit_files(
    repo: &Path,
    files: &[(&str, &str)],
    message: &str,
) -> String {
    for (file_path, contents) in files {
        let full_path = repo.join(file_path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).expect("parent dir should exist");
        }
        fs::write(full_path, contents).expect("file should be written");
    }
    run_git(repo, &["add", "."]);
    run_git(repo, &["commit", "-m", message]);
    git_output(repo, &["rev-parse", "HEAD"]).trim().to_string()
}

fn run_git(
    repo: &Path,
    args: &[&str],
) {
    let status = Command::new("git")
        .args(args)
        .current_dir(repo)
        .status()
        .expect("git command should run");
    assert!(status.success(), "git {:?} failed", args);
}

fn git_output(
    repo: &Path,
    args: &[&str],
) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("git command should run");
    assert!(output.status.success(), "git {:?} failed", args);
    String::from_utf8(output.stdout).expect("git output should be utf8")
}

fn git_ref_exists(
    repo: &Path,
    ref_name: &str,
) -> bool {
    Command::new("git")
        .args(["rev-parse", "--verify", "--quiet", ref_name])
        .current_dir(repo)
        .status()
        .expect("git command should run")
        .success()
}

#[test]
fn transplant_rewrites_selected_tree_to_branch_root() {
    let fixture = setup_branch_root_fixture();

    let dry_run_output = run(CraneCli {
        command: CraneCommand::Transplant(TransplantArgs {
            source_repo: fixture.source_repo.clone(),
            target_repo: fixture.target_repo.clone(),
            source_ref: "HEAD".to_string(),
            target_branch: "main".to_string(),
            import_branch: "crane/context-stack-root-review".to_string(),
            anchor_commit: None,
            mappings: vec![
                parse_mapping("crates/context-stack=")
                    .expect("branch-root mapping should parse"),
            ],
            no_merge: false,
            dry_run: true,
        }),
    })
    .expect("branch-root dry-run should render");
    assert!(dry_run_output.contains("mapping=crates/context-stack="));
    assert!(dry_run_output.contains("dry_run=true"));

    let outcome = execute(TransplantArgs {
        source_repo: fixture.source_repo.clone(),
        target_repo: fixture.target_repo.clone(),
        source_ref: "HEAD".to_string(),
        target_branch: "main".to_string(),
        import_branch: "crane/context-stack-root".to_string(),
        anchor_commit: None,
        mappings: vec![
            parse_mapping("crates/context-stack=")
                .expect("branch-root mapping should parse"),
        ],
        no_merge: false,
        dry_run: false,
    })
    .expect("branch-root transplant should succeed");

    assert!(outcome.merged);
    assert_eq!(outcome.plan.anchor_commit, fixture.anchor_commit);
    assert_eq!(outcome.plan.source_commit, fixture.head_commit);
    assert_eq!(
        fs::read_to_string(fixture.target_repo.join("Cargo.toml"))
            .expect("root Cargo.toml should exist"),
        "[package]\nname = \"context-stack\"\nversion = \"0.1.0\"\nedition = \"2024\"\n"
    );
    assert_eq!(
        fs::read_to_string(fixture.target_repo.join("src/lib.rs"))
            .expect("root src/lib.rs should exist"),
        "pub fn context_stack() { println!(\"branch-root\"); }\n"
    );
    assert!(
        fixture.target_repo.join("tests/smoke.rs").exists(),
        "nested files should be rewritten relative to branch root"
    );
    assert!(
        !fixture.target_repo.join("src/old.rs").exists(),
        "deletes inside the selected subtree should carry into branch root"
    );
    assert!(
        !fixture.target_repo.join("crates/context-stack").exists(),
        "selected subtree should collapse to root, not keep its source prefix"
    );
    assert!(
        !fixture
            .target_repo
            .join("crates/context-stack-extra")
            .exists(),
        "excluded sibling subtrees must stay out of the target"
    );

    let root_log = git_output(
        &fixture.target_repo,
        &[
            "log",
            "--format=%s",
            "--all",
            "--",
            "Cargo.toml",
            "src",
            "tests",
        ],
    );
    assert!(root_log.contains("context-stack scaffold"));
    assert!(root_log.contains("context-stack root updates"));
    assert!(
        !root_log.contains("sibling crate only update"),
        "commits affecting only excluded siblings must stay out of the root rewrite"
    );
}

#[test]
fn transplant_rejects_dirty_target_before_replacing_import_ref() {
    let fixture = setup_branch_root_fixture();
    let import_branch = "crane/context-stack-root-review";
    let import_ref = "refs/heads/crane/context-stack-root-review";

    execute(TransplantArgs {
        source_repo: fixture.source_repo.clone(),
        target_repo: fixture.target_repo.clone(),
        source_ref: "HEAD".to_string(),
        target_branch: "main".to_string(),
        import_branch: import_branch.to_string(),
        anchor_commit: None,
        mappings: vec![
            parse_mapping("crates/context-stack=")
                .expect("branch-root mapping should parse"),
        ],
        no_merge: true,
        dry_run: false,
    })
    .expect("initial branch-root import should succeed");

    let import_head_before =
        git_output(&fixture.target_repo, &["rev-parse", import_ref])
            .trim()
            .to_string();
    fs::write(fixture.target_repo.join("DIRTY.md"), "dirty\n")
        .expect("dirty file should be written");

    let error = execute(TransplantArgs {
        source_repo: fixture.source_repo.clone(),
        target_repo: fixture.target_repo.clone(),
        source_ref: "HEAD".to_string(),
        target_branch: "main".to_string(),
        import_branch: import_branch.to_string(),
        anchor_commit: None,
        mappings: vec![
            parse_mapping("crates/context-stack=")
                .expect("branch-root mapping should parse"),
        ],
        no_merge: true,
        dry_run: false,
    })
    .expect_err("dirty targets must be rejected before import");

    assert!(matches!(error, CraneError::DirtyTargetRepo(_)));
    assert!(git_ref_exists(&fixture.target_repo, import_ref));
    assert_eq!(
        git_output(&fixture.target_repo, &["rev-parse", import_ref])
            .trim()
            .to_string(),
        import_head_before,
        "dirty-target rejection must not delete or rewrite the existing import ref"
    );
}

#[test]
fn transplant_replaces_preexisting_import_ref_before_branch_root_import() {
    let fixture = setup_branch_root_fixture();
    let import_branch = "crane/context-stack-root-review";
    let import_ref = "refs/heads/crane/context-stack-root-review";

    execute(TransplantArgs {
        source_repo: fixture.source_repo.clone(),
        target_repo: fixture.target_repo.clone(),
        source_ref: "HEAD".to_string(),
        target_branch: "main".to_string(),
        import_branch: import_branch.to_string(),
        anchor_commit: None,
        mappings: vec![
            parse_mapping("crates/context-stack=")
                .expect("branch-root mapping should parse"),
        ],
        no_merge: true,
        dry_run: false,
    })
    .expect("initial branch-root import should succeed");

    run_git(&fixture.target_repo, &["checkout", import_branch]);
    commit_file(
        &fixture.target_repo,
        "STALE_IMPORT.md",
        "stale\n",
        "stale import branch commit",
    );
    let stale_import_head =
        git_output(&fixture.target_repo, &["rev-parse", "HEAD"])
            .trim()
            .to_string();
    run_git(&fixture.target_repo, &["checkout", "main"]);

    execute(TransplantArgs {
        source_repo: fixture.source_repo.clone(),
        target_repo: fixture.target_repo.clone(),
        source_ref: "HEAD".to_string(),
        target_branch: "main".to_string(),
        import_branch: import_branch.to_string(),
        anchor_commit: None,
        mappings: vec![
            parse_mapping("crates/context-stack=")
                .expect("branch-root mapping should parse"),
        ],
        no_merge: true,
        dry_run: false,
    })
    .expect("rerunning branch-root import should replace the stale import ref");

    let refreshed_import_head =
        git_output(&fixture.target_repo, &["rev-parse", import_ref])
            .trim()
            .to_string();
    let import_tree = git_output(
        &fixture.target_repo,
        &["ls-tree", "-r", "--name-only", import_ref],
    );

    assert_ne!(
        refreshed_import_head, stale_import_head,
        "existing import refs should be replaced, not extended in place"
    );
    assert!(
        !import_tree.contains("STALE_IMPORT.md"),
        "stale files from a previous import branch must not survive ref replacement"
    );
    assert!(import_tree.contains("Cargo.toml"));
    assert!(import_tree.contains("src/lib.rs"));
    assert!(import_tree.contains("tests/smoke.rs"));
}

struct BranchRootFixture {
    _temp: TempDir,
    source_repo: PathBuf,
    target_repo: PathBuf,
    anchor_commit: String,
    head_commit: String,
}

fn setup_branch_root_fixture() -> BranchRootFixture {
    let temp = TempDir::new().expect("tempdir should exist");
    let source_repo = temp.path().join("source");
    let target_repo = temp.path().join("target");

    init_repo(&source_repo);
    commit_file(&source_repo, "README.md", "root\n", "initial root commit");
    let anchor_commit = commit_files(
        &source_repo,
        &[
            (
                "crates/context-stack/Cargo.toml",
                "[package]\nname = \"context-stack\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
            ),
            (
                "crates/context-stack/src/lib.rs",
                "pub fn context_stack() {}\n",
            ),
            ("crates/context-stack/src/old.rs", "pub fn old() {}\n"),
            (
                "crates/context-stack-extra/Cargo.toml",
                "[package]\nname = \"context-stack-extra\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
            ),
            (
                "crates/context-stack-extra/src/lib.rs",
                "pub fn extra() {}\n",
            ),
        ],
        "context-stack scaffold",
    );
    commit_file(
        &source_repo,
        "crates/context-stack-extra/src/lib.rs",
        "pub fn extra() { println!(\"extra\"); }\n",
        "sibling crate only update",
    );
    let _ =
        fs::remove_file(source_repo.join("crates/context-stack/src/old.rs"));
    let head_commit = commit_files(
        &source_repo,
        &[
            (
                "crates/context-stack/src/lib.rs",
                "pub fn context_stack() { println!(\"branch-root\"); }\n",
            ),
            (
                "crates/context-stack/tests/smoke.rs",
                "#[test]\nfn smoke() { assert!(true); }\n",
            ),
        ],
        "context-stack root updates",
    );

    init_repo(&target_repo);
    commit_file(&target_repo, "UNRELATED.md", "target\n", "target init");

    BranchRootFixture {
        _temp: temp,
        source_repo,
        target_repo,
        anchor_commit,
        head_commit,
    }
}
