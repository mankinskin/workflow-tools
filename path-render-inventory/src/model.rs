use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ledger {
    pub schema: u32,
    pub source_digest: String,
    pub repositories: Vec<Repository>,
    pub files: Vec<FileReview>,
    pub owners: Vec<Owner>,
    #[serde(default)]
    pub publication: Option<Publication>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Repository {
    pub path: String,
    pub group: String,
    pub reviewed: bool,
    pub rationale: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileReview {
    pub id: String,
    pub repository: String,
    pub path: String,
    pub source_digest: String,
    pub origin: String,
    pub inspected: bool,
    pub owner: Option<String>,
    pub disposition: Option<Disposition>,
    pub rationale: String,
    pub gaps: Vec<Gap>,
    pub occurrences: Vec<Occurrence>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Gap {
    pub id: String,
    pub reason: String,
    pub resolution: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Occurrence {
    pub id: String,
    pub kind: String,
    pub line: usize,
    pub column: usize,
    pub context: String,
    #[serde(default)]
    pub rendering_context: String,
    pub disposition: Option<Disposition>,
    pub rationale: String,
    pub implemented: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Disposition {
    Migrate,
    PreserveFormat,
    Generated,
    NoOccurrence,
    Exception,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Owner {
    pub id: String,
    pub manifest: String,
    pub package_manifest: String,
    pub package: String,
    pub test_args: Vec<String>,
    pub linux_recipe: String,
    pub windows: WindowsApplicability,
    pub windows_test_args: Vec<String>,
    pub windows_recipe: String,
    pub windows_rationale: String,
    pub dependency_revision: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum WindowsApplicability {
    Pending,
    Applicable,
    Inapplicable,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Publication {
    pub revision: String,
    pub consumer_owners: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Stage {
    Inventory,
    Migration,
    Final,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Linux,
    Windows,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Selection {
    pub selector: String,
    pub stage: Stage,
    pub file_ids: Vec<String>,
    pub repository_paths: Vec<String>,
    pub file_sources: Vec<FileSource>,
    pub occurrence_ids: Vec<String>,
    pub owner_ids: Vec<String>,
    pub digest: String,
    pub aggregate: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSource {
    pub path: String,
    pub hash: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Source {
    pub root_revision: String,
    pub source_digest: String,
    pub gitlinks: Vec<String>,
    pub files: Vec<SourceFile>,
    pub exclusions: Vec<Exclusion>,
    #[serde(default)]
    pub recipe_digests: std::collections::BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceFile {
    pub path: String,
    pub hash: String,
    pub untracked: bool,
    #[serde(default)]
    pub cargo_packages: Vec<LockPackage>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LockPackage {
    pub name: String,
    pub version: String,
    pub hash: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Exclusion {
    pub path: String,
    pub reason: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OwnerProof {
    pub owner: String,
    pub platform: Platform,
    pub content_digest: String,
    pub test_args: Vec<String>,
    pub recipe: String,
    pub passed: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OwnerRun {
    pub owner: String,
    pub metadata_file: String,
    pub proof: OwnerProof,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidationResult {
    pub selection: Selection,
    pub owners: Vec<OwnerRun>,
    pub publication_revision: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CargoGraph {
    pub workspace_root: String,
    pub packages: Vec<CargoPackage>,
    pub resolve: CargoResolve,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CargoPackage {
    pub id: String,
    pub name: String,
    pub version: String,
    pub manifest_path: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CargoResolve {
    pub nodes: Vec<CargoNode>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CargoNode {
    pub id: String,
    pub dependencies: Vec<String>,
}
