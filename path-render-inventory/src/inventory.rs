use crate::{Result, hash, input_path, model::*};
use proc_macro2::Span;
use std::{collections::BTreeMap, fs, path::Path};
use syn::{spanned::Spanned, visit::Visit};

fn group(repository: &str) -> String {
    match repository.split('/').next().unwrap_or("") {
        "workflow-tools" => "workflow-tools",
        "context-engine" => "context-engine",
        _ => "remaining",
    }
    .to_owned()
}

fn owner_for(root: &Path, file: &str) -> Result<Option<Owner>> {
    let mut parent = Path::new(file).parent();
    let mut package = None;
    let mut workspace = None;
    while let Some(directory) = parent {
        let manifest = directory.join("Cargo.toml");
        if root.join(&manifest).is_file() {
            let relative = manifest
                .components()
                .map(|part| part.as_os_str().to_str().ok_or("non-UTF manifest path"))
                .collect::<std::result::Result<Vec<_>, _>>()?
                .join("/");
            let value: toml::Value =
                toml::from_str(&fs::read_to_string(input_path(root, &relative)?)?)?;
            if package.is_none()
                && let Some(name) = value
                    .get("package")
                    .and_then(|v| v.get("name"))
                    .and_then(|v| v.as_str())
            {
                package = Some((relative.clone(), name.to_owned()));
            }
            if value.get("workspace").is_some() && workspace.is_none() {
                workspace = Some(relative);
            }
        }
        parent = directory.parent();
    }
    let Some((package_manifest, package)) = package else {
        return Ok(None);
    };
    let manifest = workspace.unwrap_or_else(|| package_manifest.clone());
    let id = hash(format!("{manifest}\n{package}"));
    Ok(Some(Owner {
        id: id[..24].to_owned(),
        manifest: manifest.clone(),
        package_manifest,
        package: package.clone(),
        test_args: vec![
            "test".into(),
            "--locked".into(),
            "--manifest-path".into(),
            manifest,
            "-p".into(),
            package,
        ],
        linux_recipe: "rust-bookworm-openssl".into(),
        windows: WindowsApplicability::Pending,
        windows_test_args: vec![],
        windows_recipe: String::new(),
        windows_rationale: String::new(),
        dependency_revision: None,
    }))
}

struct Candidates<'a> {
    file: &'a str,
    source: &'a str,
    occurrences: Vec<Occurrence>,
    duplicates: BTreeMap<String, usize>,
}

impl Candidates<'_> {
    fn record(&mut self, kind: &str, span: Span) {
        let start = span.start();
        let context = self
            .source
            .lines()
            .nth(start.line.saturating_sub(1))
            .unwrap_or("")
            .trim();
        let anchor = format!("{}\n{kind}\n{context}", self.file);
        let duplicate = self.duplicates.entry(anchor.clone()).or_default();
        let id = hash(format!("{anchor}\n{duplicate}"));
        *duplicate += 1;
        self.occurrences.push(Occurrence {
            id: id[..24].into(),
            kind: kind.into(),
            line: start.line,
            column: start.column,
            context: context.into(),
            rendering_context: String::new(),
            disposition: None,
            rationale: String::new(),
            implemented: false,
        });
    }
}

impl<'ast> Visit<'ast> for Candidates<'_> {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let name = node.method.to_string();
        if name.starts_with("to_")
            || matches!(
                name.as_str(),
                "display"
                    | "as_str"
                    | "as_os_str"
                    | "into_string"
                    | "into_owned"
                    | "join"
                    | "serialize"
                    | "write_fmt"
            )
        {
            self.record("conversion-or-wrapper", node.method.span());
        }
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = node.func.as_ref()
            && path.path.segments.iter().any(|part| {
                let name = part.ident.to_string().to_lowercase();
                ["path", "render", "string", "format", "serialize", "error"]
                    .iter()
                    .any(|needle| name.contains(needle))
            })
        {
            self.record("formatting-or-wrapper-call", node.span());
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.record("opaque-macro-or-formatting", node.span());
        syn::visit::visit_macro(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        if let Some((_, path, _)) = &node.trait_
            && path.segments.iter().any(|part| {
                matches!(
                    part.ident.to_string().as_str(),
                    "Display" | "Debug" | "Serialize" | "Deserialize"
                )
            })
        {
            self.record("formatting-or-serde-implementation", node.span());
        }
        syn::visit::visit_item_impl(self, node);
    }

    fn visit_attribute(&mut self, node: &'ast syn::Attribute) {
        if node.path().is_ident("derive") {
            self.record("derive-or-generated-format", node.span());
        }
        syn::visit::visit_attribute(self, node);
    }
}

pub fn scan(root: &Path, source: &Source) -> Result<Ledger> {
    let mut repositories = vec![Repository {
        path: String::new(),
        group: "remaining".into(),
        reviewed: false,
        rationale: String::new(),
    }];
    for link in &source.gitlinks {
        let path = link.get(42..).ok_or("invalid recursive gitlink receipt")?;
        let path = path.split(" (").next().ok_or("invalid gitlink path")?;
        repositories.push(Repository {
            path: path.into(),
            group: group(path),
            reviewed: false,
            rationale: String::new(),
        });
    }
    repositories.sort_by(|a, b| a.path.cmp(&b.path));
    repositories.dedup_by(|a, b| a.path == b.path);
    let repository_for = |path: &str| {
        repositories
            .iter()
            .filter(|repo| repo.path.is_empty() || path.starts_with(&(repo.path.clone() + "/")))
            .max_by_key(|repo| repo.path.len())
            .map(|repo| repo.path.clone())
            .unwrap_or_default()
    };
    let mut owners = BTreeMap::new();
    let mut files = vec![];
    for input in source
        .files
        .iter()
        .filter(|input| input.path.ends_with(".rs"))
    {
        let bytes = fs::read(input_path(root, &input.path)?)?;
        if hash(&bytes) != input.hash {
            return Err(format!("copied source mismatch: {}", input.path).into());
        }
        let text = std::str::from_utf8(&bytes)?;
        let owner = owner_for(root, &input.path)?;
        let owner_id = owner.as_ref().map(|owner| owner.id.clone());
        if let Some(owner) = owner {
            owners.insert(owner.id.clone(), owner);
        }
        let id = hash(&input.path)[..24].to_owned();
        let mut gaps = vec![Gap {
            id: format!("{id}:semantic"), reason: "Whole-file semantic review required: arbitrary wrappers, errors, serde, macros and generated code can hide path conversion.".into(),
            resolution: String::new(),
        }];
        let mut visitor = Candidates {
            file: &input.path,
            source: text,
            occurrences: vec![],
            duplicates: BTreeMap::new(),
        };
        match syn::parse_file(text) {
            Ok(parsed) => visitor.visit_file(&parsed),
            Err(error) => gaps.push(Gap {
                id: format!("{id}:parse"),
                reason: format!("Rust parsing failed: {error}"),
                resolution: String::new(),
            }),
        }
        if owner_id.is_none() {
            gaps.push(Gap { id: format!("{id}:owner"), reason: "No Cargo owner inferred; assign verified ownership or justify a generated/exception policy.".into(), resolution: String::new() });
        }
        files.push(FileReview {
            id,
            repository: repository_for(&input.path),
            path: input.path.clone(),
            source_digest: input.hash.clone(),
            origin: if input.untracked {
                "untracked"
            } else {
                "tracked"
            }
            .into(),
            inspected: false,
            owner: owner_id,
            disposition: None,
            rationale: String::new(),
            gaps,
            occurrences: visitor.occurrences,
        });
    }
    for excluded in source
        .exclusions
        .iter()
        .filter(|input| input.path.ends_with(".rs"))
    {
        let id = hash(&excluded.path)[..24].to_owned();
        files.push(FileReview {
            id: id.clone(),
            repository: repository_for(&excluded.path),
            path: excluded.path.clone(),
            source_digest: String::new(),
            origin: excluded.reason.clone(),
            inspected: false,
            owner: None,
            disposition: None,
            rationale: String::new(),
            occurrences: vec![],
            gaps: vec![Gap {
                id: format!("{id}:excluded"),
                reason: format!(
                    "Excluded Rust input needs an explicit generated/deletion policy: {}",
                    excluded.reason
                ),
                resolution: String::new(),
            }],
        });
    }
    files.sort_by(|a, b| (&a.repository, &a.path).cmp(&(&b.repository, &b.path)));
    Ok(Ledger {
        schema: 1,
        source_digest: source.source_digest.clone(),
        repositories,
        files,
        owners: owners.into_values().collect(),
        publication: None,
    })
}

pub fn selections(ledger: &Ledger, stage: Stage, windows: bool) -> Result<Vec<Selection>> {
    let mut result = vec![];
    for group in ["workflow-tools", "context-engine", "remaining"] {
        let mut units: Vec<(String, Option<String>)> = vec![];
        let mut files: Vec<_> = ledger
            .files
            .iter()
            .filter(|file| {
                ledger
                    .repositories
                    .iter()
                    .any(|repo| repo.path == file.repository && repo.group == group)
                    && (!windows
                        || file.owner.as_ref().is_some_and(|id| {
                            ledger.owners.iter().any(|owner| {
                                &owner.id == id && owner.windows == WindowsApplicability::Applicable
                            })
                        }))
            })
            .collect();
        files.sort_by(|a, b| (&a.repository, &a.path).cmp(&(&b.repository, &b.path)));
        for file in files {
            if file.occurrences.is_empty() {
                units.push((file.id.clone(), None));
            }
            let mut occurrences: Vec<_> = file.occurrences.iter().collect();
            occurrences.sort_by(|a, b| (a.line, a.column, &a.id).cmp(&(b.line, b.column, &b.id)));
            units.extend(
                occurrences
                    .into_iter()
                    .map(|row| (file.id.clone(), Some(row.id.clone()))),
            );
        }
        let mut cursor = 0;
        let mut number = 1;
        if units.is_empty()
            && !windows
            && ledger.repositories.iter().any(|repo| repo.group == group)
        {
            result.push(make_selection(
                ledger,
                format!("batch:{group}:001"),
                stage,
                vec![],
                vec![],
                false,
            )?);
        }
        while cursor < units.len() {
            let mut file_ids = vec![];
            let mut occurrence_ids = vec![];
            let start = cursor;
            while cursor < units.len() && cursor - start < 100 {
                let (file, occurrence) = &units[cursor];
                if !file_ids.contains(file) {
                    if file_ids.len() == 25 {
                        break;
                    }
                    file_ids.push(file.clone());
                }
                if let Some(row) = occurrence {
                    occurrence_ids.push(row.clone());
                }
                cursor += 1;
            }
            let prefix = if windows { "windows-batch" } else { "batch" };
            result.push(make_selection(
                ledger,
                format!("{prefix}:{group}:{number:03}"),
                stage,
                file_ids,
                occurrence_ids,
                false,
            )?);
            number += 1;
        }
    }
    Ok(result)
}

pub fn make_selection(
    ledger: &Ledger,
    selector: String,
    stage: Stage,
    file_ids: Vec<String>,
    occurrence_ids: Vec<String>,
    aggregate: bool,
) -> Result<Selection> {
    let selected: Vec<_> = ledger
        .files
        .iter()
        .filter(|file| file_ids.contains(&file.id))
        .collect();
    let mut owner_ids: Vec<_> = selected
        .iter()
        .filter_map(|file| file.owner.clone())
        .collect();
    owner_ids.sort();
    owner_ids.dedup();
    let owners: Vec<_> = ledger
        .owners
        .iter()
        .filter(|owner| owner_ids.contains(&owner.id))
        .collect();
    let group = selector.split(':').nth(1);
    let repositories: Vec<_> = ledger
        .repositories
        .iter()
        .filter(|repo| {
            if aggregate {
                return group.is_none_or(|group| repo.group == group);
            }
            selected.iter().any(|file| file.repository == repo.path)
                || (!selector.starts_with("windows-batch:")
                    && selector.ends_with(":001")
                    && group == Some(repo.group.as_str())
                    && !ledger.files.iter().any(|file| file.repository == repo.path))
        })
        .collect();
    let repository_paths = repositories.iter().map(|repo| repo.path.clone()).collect();
    let digest = hash(serde_json::to_vec(&(
        &selector,
        stage,
        &selected,
        &owners,
        &occurrence_ids,
        &repositories,
    ))?);
    let file_sources = selected
        .iter()
        .map(|file| FileSource {
            path: file.path.clone(),
            hash: file.source_digest.clone(),
        })
        .collect();
    Ok(Selection {
        selector,
        stage,
        file_ids,
        repository_paths,
        file_sources,
        occurrence_ids,
        owner_ids,
        digest,
        aggregate,
    })
}
