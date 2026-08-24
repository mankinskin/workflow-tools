use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use anyhow::{
    Context,
    Result,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub template_dir: PathBuf,
    pub template: String,
    pub model: Option<String>,
    pub file_root: PathBuf,
    pub ticket_workspace: PathBuf,
    pub ticket_mcp_command: String,
    #[serde(default)]
    pub ticket_mcp_args: Vec<String>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path).with_context(|| {
            format!("failed to read config file {}", path.display())
        })?;
        let mut config = Self::from_toml(&contents)?;
        config.resolve_paths(path.parent().unwrap_or_else(|| Path::new(".")));
        Ok(config)
    }

    pub fn from_toml(contents: &str) -> Result<Self> {
        toml::from_str(contents).context("failed to parse configuration TOML")
    }

    pub fn resolve_paths(
        &mut self,
        config_dir: &Path,
    ) {
        if self.template_dir.is_relative() {
            self.template_dir = config_dir.join(&self.template_dir);
        }
        if self.file_root.is_relative() {
            self.file_root = config_dir.join(&self.file_root);
        }
        if self.ticket_workspace.is_relative() {
            self.ticket_workspace = config_dir.join(&self.ticket_workspace);
        }
    }

    pub fn template_path(&self) -> PathBuf {
        let template = Path::new(&self.template);
        if template.is_absolute() {
            template.to_path_buf()
        } else {
            self.template_dir.join(template)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::{
        Path,
        PathBuf,
    };

    use super::Config;

    #[test]
    fn parses_config_and_resolves_template_directory() {
        let mut config = Config::from_toml(
            r#"
template_dir = "templates"
template = "age.md"
model = "gpt-5.3-codex"
file_root = "files"
ticket_workspace = "ticket-store"
ticket_mcp_command = "mcp-toolmon"
ticket_mcp_args = ["--", "ticket-mcp"]
"#,
        )
        .unwrap();

        config.resolve_paths(Path::new("fixtures/request"));

        assert_eq!(
            config.template_dir,
            PathBuf::from("fixtures/request/templates")
        );
        assert_eq!(config.model.as_deref(), Some("gpt-5.3-codex"));
        assert_eq!(config.file_root, PathBuf::from("fixtures/request/files"));
        assert_eq!(
            config.ticket_workspace,
            PathBuf::from("fixtures/request/ticket-store")
        );
    }

    #[test]
    fn resolves_template_name_under_template_directory() {
        let config = Config {
            template_dir: PathBuf::from("fixtures/templates"),
            template: "age-lookup.md".to_owned(),
            model: None,
            file_root: PathBuf::from("fixtures/files"),
            ticket_workspace: PathBuf::from("fixtures/tickets"),
            ticket_mcp_command: "mcp-toolmon".to_owned(),
            ticket_mcp_args: vec!["--".to_owned(), "ticket-mcp".to_owned()],
        };

        assert_eq!(
            config.template_path(),
            PathBuf::from("fixtures/templates/age-lookup.md")
        );
    }
}
