use crate::{Result, hash, inventory, model::*};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    path::Path,
};

pub fn select(ledger: &Ledger, selector: &str, stage: Stage) -> Result<Selection> {
    if selector.starts_with("batch:") || selector.starts_with("windows-batch:") {
        let windows = selector.starts_with("windows-batch:");
        if windows && stage != Stage::Final {
            return Err("Windows batches require final stage".into());
        }
        return inventory::selections(ledger, stage, windows)?
            .into_iter()
            .find(|selection| selection.selector == selector)
            .ok_or_else(|| format!("unknown batch: {selector}").into());
    }
    let expected = match selector {
        "inventory-complete" => Stage::Inventory,
        "linux-migration-complete" => Stage::Migration,
        "final-both-platform-complete" => Stage::Final,
        _ if selector.starts_with("inventory-group:") && selector.ends_with(":complete") => {
            Stage::Inventory
        }
        _ if selector.starts_with("group:") && selector.ends_with(":complete") => Stage::Migration,
        _ => return Err(format!("unknown aggregate selector: {selector}").into()),
    };
    if expected != stage {
        return Err("selector does not belong to the requested stage".into());
    }
    let group = if selector.contains(':') {
        selector.split(':').nth(1)
    } else {
        None
    };
    if group.is_some_and(|group| !ledger.repositories.iter().any(|repo| repo.group == group)) {
        return Err("unknown repository group".into());
    }
    let selected: Vec<_> = ledger
        .files
        .iter()
        .filter(|file| {
            group.is_none_or(|group| {
                ledger
                    .repositories
                    .iter()
                    .any(|repo| repo.path == file.repository && repo.group == group)
            })
        })
        .collect();
    inventory::make_selection(
        ledger,
        selector.into(),
        stage,
        selected.iter().map(|file| file.id.clone()).collect(),
        selected
            .iter()
            .flat_map(|file| file.occurrences.iter().map(|row| row.id.clone()))
            .collect(),
        true,
    )
}

fn unique<'a>(values: impl Iterator<Item = &'a str>, label: &str) -> Result<()> {
    let mut seen = BTreeSet::new();
    for value in values {
        if value.is_empty() || !seen.insert(value) {
            return Err(format!("empty or duplicate {label}: {value}").into());
        }
    }
    Ok(())
}

fn require_text(text: &str, label: &str) -> Result<()> {
    if text.trim().is_empty() {
        return Err(format!("missing {label}").into());
    }
    Ok(())
}

pub fn test_arguments(owner: &Owner, platform: Platform) -> Result<&[String]> {
    let args = match platform {
        Platform::Linux => &owner.test_args,
        Platform::Windows => &owner.windows_test_args,
    };
    if !matches!(args.first().map(String::as_str), Some("test" | "clippy"))
        || !args.iter().any(|arg| arg == "--locked")
        || !args
            .windows(2)
            .any(|pair| pair[0] == "--manifest-path" && pair[1] == owner.manifest)
        || !args
            .windows(2)
            .any(|pair| pair[0] == "-p" && pair[1] == owner.package)
    {
        return Err(format!("unowned or incomplete Cargo arguments for {}", owner.id).into());
    }
    if args
        .iter()
        .filter(|arg| arg.as_str() == "--manifest-path")
        .count()
        != 1
        || args.iter().filter(|arg| arg.as_str() == "-p").count() != 1
        || args.iter().any(|arg| {
            arg == "--workspace"
                || arg == "--exclude"
                || arg == "--package"
                || arg.starts_with("--manifest-path=")
                || arg.starts_with("--package=")
                || arg.starts_with("--exclude=")
                || (arg.starts_with("-p") && arg != "-p")
        })
    {
        return Err("ambiguous or owner-excluding Cargo arguments".into());
    }
    for path in [&owner.manifest, &owner.package_manifest] {
        if path.is_empty()
            || path.contains('\\')
            || path.starts_with('/')
            || path.split('/').any(|part| matches!(part, "" | "." | ".."))
        {
            return Err(format!("invalid owner manifest: {path}").into());
        }
    }
    require_text(&owner.package, "owner package")?;
    Ok(args)
}

pub fn check_schema(ledger: &Ledger, source: &Source) -> Result<()> {
    if ledger.schema != 1 {
        return Err("unsupported ledger schema".into());
    }
    unique(ledger.files.iter().map(|file| file.id.as_str()), "file id")?;
    unique(
        ledger.files.iter().map(|file| file.path.as_str()),
        "source path",
    )?;
    unique(
        ledger.owners.iter().map(|owner| owner.id.as_str()),
        "owner id",
    )?;
    unique(
        ledger
            .files
            .iter()
            .flat_map(|file| file.occurrences.iter().map(|row| row.id.as_str())),
        "occurrence id",
    )?;
    let actual: BTreeSet<_> = source
        .files
        .iter()
        .filter(|file| file.path.ends_with(".rs"))
        .map(|file| &file.path)
        .chain(
            source
                .exclusions
                .iter()
                .filter(|file| file.path.ends_with(".rs"))
                .map(|file| &file.path),
        )
        .collect();
    let declared: BTreeSet<_> = ledger.files.iter().map(|file| &file.path).collect();
    if actual != declared {
        return Err(
            "Rust file/coverage census mismatch; re-inventory additions and deletions".into(),
        );
    }
    let actual_repos: BTreeSet<_> = source
        .gitlinks
        .iter()
        .map(|link| {
            link.get(42..)
                .ok_or("invalid gitlink receipt")
                .map(|path| path.split(" (").next().unwrap_or(path).to_owned())
        })
        .collect::<std::result::Result<_, _>>()?;
    let declared_repos: BTreeSet<_> = ledger
        .repositories
        .iter()
        .filter(|repo| !repo.path.is_empty())
        .map(|repo| repo.path.clone())
        .collect();
    if actual_repos != declared_repos
        || ledger.repositories.len() != declared_repos.len() + 1
        || ledger
            .repositories
            .iter()
            .filter(|repo| repo.path.is_empty())
            .count()
            != 1
    {
        return Err("recursive repository census mismatch".into());
    }
    for repository in &ledger.repositories {
        let expected = match repository.path.split('/').next().unwrap_or("") {
            "workflow-tools" => "workflow-tools",
            "context-engine" => "context-engine",
            _ => "remaining",
        };
        if repository.group != expected {
            return Err("repository group does not match the recursive scope".into());
        }
    }
    for owner in &ledger.owners {
        test_arguments(owner, Platform::Linux)?;
        require_text(&owner.linux_recipe, "Linux dependency recipe")?;
    }
    for file in &ledger.files {
        if !ledger
            .repositories
            .iter()
            .any(|repo| repo.path == file.repository)
        {
            return Err(format!("unowned repository: {}", file.path).into());
        }
        if file
            .owner
            .as_ref()
            .is_some_and(|id| !ledger.owners.iter().any(|owner| &owner.id == id))
        {
            return Err(format!("unknown owner: {}", file.path).into());
        }
        unique(
            file.gaps.iter().map(|gap| gap.id.as_str()),
            "coverage gap id",
        )?;
        for row in &file.occurrences {
            if row.line == 0 {
                return Err(format!("missing occurrence coordinate: {}", row.id).into());
            }
        }
    }
    Ok(())
}

pub fn check_classification(ledger: &Ledger, source: &Source, selection: &Selection) -> Result<()> {
    for file in ledger
        .files
        .iter()
        .filter(|file| selection.file_ids.contains(&file.id))
    {
        if !file.inspected {
            return Err(format!("file has not been semantically inspected: {}", file.path).into());
        }
        require_text(&file.rationale, "whole-file review rationale")?;
        for gap in &file.gaps {
            require_text(&gap.resolution, &format!("coverage resolution {}", gap.id))?;
        }
        if let Some(current) = source
            .files
            .iter()
            .find(|current| current.path == file.path)
        {
            if current.hash != file.source_digest {
                return Err(format!("stale file classification: {}", file.path).into());
            }
        } else if !matches!(
            file.disposition,
            Some(Disposition::Generated | Disposition::Exception)
        ) {
            return Err("excluded input needs a justified generated/exception policy".into());
        }
        if file.owner.is_none()
            && !matches!(
                file.disposition,
                Some(Disposition::Generated | Disposition::Exception)
            )
        {
            return Err(format!("classified Rust file has no test owner: {}", file.path).into());
        }
        if file.occurrences.is_empty() && file.disposition.is_none() {
            return Err("zero-candidate file needs a disposition".into());
        }
        for row in file
            .occurrences
            .iter()
            .filter(|row| selection.occurrence_ids.contains(&row.id))
        {
            if row.disposition == Some(Disposition::Migrate) && file.owner.is_none() {
                return Err(
                    format!("eligible occurrence has no concrete test owner: {}", row.id).into(),
                );
            }
            if row.disposition.is_none() {
                return Err(format!("unclassified occurrence: {}", row.id).into());
            }
            require_text(&row.rationale, "occurrence rationale")?;
            require_text(
                &row.rendering_context,
                "semantic rendering context (human/machine/filesystem/portable/identity)",
            )?;
            if selection.stage != Stage::Inventory
                && row.disposition == Some(Disposition::Migrate)
                && !row.implemented
            {
                return Err(format!("migration is not implemented: {}", row.id).into());
            }
        }
    }
    for repo in ledger
        .repositories
        .iter()
        .filter(|repo| selection.repository_paths.contains(&repo.path))
    {
        if !repo.reviewed {
            return Err(format!("repository census not reviewed: {}", repo.path).into());
        }
        require_text(
            &repo.rationale,
            "repository coverage rationale, including zero-Rust repositories",
        )?;
    }
    Ok(())
}

pub fn check_consumer_pin(
    ledger: &Ledger,
    owner: &Owner,
    root: &Path,
    workspace: &Path,
) -> Result<()> {
    let publication = ledger
        .publication
        .as_ref()
        .ok_or("missing immutable renderer publication")?;
    if !publication.consumer_owners.contains(&owner.id) {
        return Ok(());
    }
    if !workspace.starts_with(root) {
        return Err("consumer workspace is outside copied source".into());
    }
    let manifest: toml::Value = toml::from_str(&std::fs::read_to_string(
        root.join(&owner.package_manifest),
    )?)?;
    let workspace_manifest: toml::Value =
        toml::from_str(&std::fs::read_to_string(workspace.join("Cargo.toml"))?)?;
    let mut found = false;
    let mut tables = vec![&manifest];
    if let Some(targets) = manifest.get("target").and_then(toml::Value::as_table) {
        tables.extend(targets.values());
    }
    for table in tables {
        for kind in ["dependencies", "dev-dependencies", "build-dependencies"] {
            let Some(dependencies) = table.get(kind).and_then(toml::Value::as_table) else {
                continue;
            };
            for (name, declared) in dependencies {
                let dependency =
                    if declared.get("workspace").and_then(toml::Value::as_bool) == Some(true) {
                        workspace_manifest
                            .get("workspace")
                            .and_then(|value| value.get("dependencies"))
                            .and_then(|value| value.get(name))
                            .ok_or("missing inherited dependency")?
                    } else {
                        declared
                    };
                if name != "path-render"
                    && dependency.get("package").and_then(toml::Value::as_str)
                        != Some("path-render")
                {
                    continue;
                }
                found = true;
                if dependency
                    .get("git")
                    .and_then(toml::Value::as_str)
                    .is_none()
                    || dependency.get("rev").and_then(toml::Value::as_str)
                        != Some(publication.revision.as_str())
                    || dependency.get("path").is_some()
                    || dependency.get("branch").is_some()
                    || dependency.get("tag").is_some()
                {
                    return Err(
                        "actual consumer manifest is not pinned to published renderer".into(),
                    );
                }
            }
        }
    }
    if !found {
        return Err("declared consumer has no actual path-render dependency".into());
    }
    Ok(())
}

pub fn owner_digest(
    owner: &Owner,
    graph: &CargoGraph,
    source: &Source,
    root: &Path,
    platform: Platform,
) -> Result<String> {
    if !root.is_absolute() {
        return Err("owner identity requires an absolute container root".into());
    }
    let package = graph
        .packages
        .iter()
        .find(|package| {
            package.name == owner.package
                && Path::new(&package.manifest_path) == root.join(&owner.package_manifest)
        })
        .ok_or("selected package is absent from resolved Cargo metadata")?;
    let mut reachable = BTreeSet::new();
    let mut queue = VecDeque::from([package.id.clone()]);
    while let Some(id) = queue.pop_front() {
        if !reachable.insert(id.clone()) {
            continue;
        }
        let node = graph
            .resolve
            .nodes
            .iter()
            .find(|node| node.id == id)
            .ok_or("missing resolved dependency node")?;
        queue.extend(node.dependencies.iter().cloned());
    }
    let mut roots = vec![];
    let mut dependencies = vec![];
    let relative = |path: &Path| -> Result<String> {
        Ok(path
            .strip_prefix(&root)?
            .components()
            .map(|part| part.as_os_str().to_str().ok_or("non-UTF Cargo path"))
            .collect::<std::result::Result<Vec<_>, _>>()?
            .join("/"))
    };
    for package in graph
        .packages
        .iter()
        .filter(|package| reachable.contains(&package.id))
    {
        if Path::new(&package.manifest_path).starts_with(&root) {
            let directory = Path::new(&package.manifest_path)
                .parent()
                .ok_or("manifest has no parent")?;
            let prefix = relative(directory)?;
            roots.push(if prefix.is_empty() {
                prefix
            } else {
                prefix + "/"
            });
            dependencies.push(format!(
                "local:{}:{}:{}",
                relative(Path::new(&package.manifest_path))?,
                package.name,
                package.version
            ));
        } else {
            dependencies.push(package.id.clone());
        }
    }
    dependencies.sort();
    let workspace = relative(Path::new(&graph.workspace_root))?;
    let prefix = if workspace.is_empty() {
        workspace
    } else {
        workspace + "/"
    };
    let mut inputs = vec![serde_json::to_string(test_arguments(owner, platform)?)?];
    let platform_name = match platform {
        Platform::Linux => "linux",
        Platform::Windows => "windows",
    };
    inputs.push(
        source
            .recipe_digests
            .get(platform_name)
            .ok_or("missing current immutable platform recipe identity")?
            .clone(),
    );
    inputs.push(match platform {
        Platform::Linux => owner.linux_recipe.clone(),
        Platform::Windows => owner.windows_recipe.clone(),
    });
    inputs.extend(dependencies);
    for file in &source.files {
        if roots.iter().any(|prefix| file.path.starts_with(prefix))
            || file.path == format!("{prefix}Cargo.toml")
            || file.path.starts_with(&format!("{prefix}.cargo/"))
            || file.path.starts_with(".cargo/")
            || ((!file.path.ends_with(".md"))
                && (file
                    .path
                    .starts_with("workflow-tools/install/docker-validation/")
                    || file
                        .path
                        .starts_with("workflow-tools/install/viewer-validation/")
                    || file
                        .path
                        .starts_with("workflow-tools/path-render-inventory/")
                    || file.path == "workflow-tools/install/validation-lib.sh"))
        {
            inputs.push(format!("{}\t{}", file.path, file.hash));
        }
    }
    let lock = source
        .files
        .iter()
        .find(|file| file.path == format!("{prefix}Cargo.lock"))
        .ok_or("missing owner lockfile")?;
    for package in graph
        .packages
        .iter()
        .filter(|package| reachable.contains(&package.id))
    {
        let sections: Vec<_> = lock
            .cargo_packages
            .iter()
            .filter(|section| section.name == package.name && section.version == package.version)
            .collect();
        if sections.is_empty() {
            return Err(format!("missing dependency lock section: {}", package.name).into());
        }
        inputs.extend(
            sections
                .into_iter()
                .map(|section| format!("{}@{}\t{}", section.name, section.version, section.hash)),
        );
    }
    Ok(hash(inputs.join("\n")))
}

pub fn check_evidence(
    ledger: &Ledger,
    selection: &Selection,
    current: &BTreeMap<(String, PlatformKey), String>,
    proofs: &[OwnerProof],
) -> Result<()> {
    check_platform_evidence(ledger, selection, current, proofs, Platform::Linux)
}

pub fn check_platform_evidence(
    ledger: &Ledger,
    selection: &Selection,
    current: &BTreeMap<(String, PlatformKey), String>,
    proofs: &[OwnerProof],
    platform: Platform,
) -> Result<()> {
    if selection.stage == Stage::Inventory {
        return Ok(());
    }
    for owner in ledger
        .owners
        .iter()
        .filter(|owner| selection.owner_ids.contains(&owner.id))
    {
        if selection.selector == "linux-migration-complete" || selection.stage == Stage::Final {
            match owner.windows {
                WindowsApplicability::Pending => {
                    return Err(format!("Windows applicability is unresolved: {}", owner.id).into());
                }
                WindowsApplicability::Inapplicable => require_text(
                    &owner.windows_rationale,
                    "Windows inapplicability rationale",
                )?,
                WindowsApplicability::Applicable => {
                    test_arguments(owner, Platform::Windows)?;
                    require_text(&owner.windows_recipe, "Windows dependency recipe")?;
                }
            }
        }
        let mut platforms = vec![if selection.selector.starts_with("windows-batch:") {
            platform
        } else {
            Platform::Linux
        }];
        if selection.selector == "final-both-platform-complete"
            && owner.windows == WindowsApplicability::Applicable
        {
            platforms.push(Platform::Windows);
        }
        for platform in platforms {
            let digest = current
                .get(&(owner.id.clone(), PlatformKey::from(platform)))
                .ok_or("missing current owner identity")?;
            let args = test_arguments(owner, platform)?;
            if !proofs.iter().any(|proof| {
                proof.owner == owner.id
                    && proof.platform == platform
                    && proof.passed
                    && &proof.content_digest == digest
                    && proof.test_args == args
                    && proof.recipe
                        == match platform {
                            Platform::Linux => &owner.linux_recipe,
                            Platform::Windows => &owner.windows_recipe,
                        }
                        .as_str()
            }) {
                return Err(
                    format!("missing/stale {platform:?} owner evidence: {}", owner.id).into(),
                );
            }
        }
    }
    if selection.stage == Stage::Final {
        let publication = ledger
            .publication
            .as_ref()
            .ok_or("missing immutable renderer publication")?;
        if publication.revision.len() != 40
            || !publication
                .revision
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
        {
            return Err("publication must identify a full Git SHA".into());
        }
        for owner in ledger
            .owners
            .iter()
            .filter(|owner| publication.consumer_owners.contains(&owner.id))
        {
            if owner.dependency_revision.as_ref() != Some(&publication.revision) {
                return Err("consumer is not pinned to latest published renderer".into());
            }
        }
        for file in &ledger.files {
            if !file.repository.starts_with("workflow-tools")
                && file
                    .occurrences
                    .iter()
                    .any(|row| row.disposition == Some(Disposition::Migrate))
                && file
                    .owner
                    .as_ref()
                    .is_none_or(|owner| !publication.consumer_owners.contains(owner))
            {
                return Err(
                    "migrated independent consumer is missing from publication coverage".into(),
                );
            }
        }
    }
    Ok(())
}

pub fn check_detected_coverage(ledger: &Ledger, fresh: &Ledger) -> Result<()> {
    for file in &fresh.files {
        let recorded = ledger
            .files
            .iter()
            .find(|recorded| recorded.path == file.path)
            .ok_or("missing Rust file")?;
        for row in &file.occurrences {
            if !recorded.occurrences.iter().any(|recorded| {
                recorded.id == row.id
                    && recorded.kind == row.kind
                    && recorded.context == row.context
            }) {
                return Err(format!(
                    "detected rendering candidate missing from ledger: {} {}",
                    file.path, row.id
                )
                .into());
            }
        }
        for gap in &file.gaps {
            if !recorded
                .gaps
                .iter()
                .any(|recorded| recorded.id == gap.id && recorded.reason == gap.reason)
            {
                return Err(
                    format!("detected coverage gap missing from ledger: {}", gap.id).into(),
                );
            }
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlatformKey {
    Linux,
    Windows,
}

impl From<Platform> for PlatformKey {
    fn from(platform: Platform) -> Self {
        match platform {
            Platform::Linux => Self::Linux,
            Platform::Windows => Self::Windows,
        }
    }
}
