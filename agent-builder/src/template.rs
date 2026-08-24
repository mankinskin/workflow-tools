use std::{
    fs,
    path::Path,
};

use anyhow::{
    Context,
    Result,
    bail,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct Frontmatter {
    model: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AgentTemplate {
    pub model: String,
    pub preamble: String,
}

impl AgentTemplate {
    pub fn load(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path).with_context(|| {
            format!("failed to read template {}", path.display())
        })?;
        Self::parse(&contents)
    }

    pub fn parse(contents: &str) -> Result<Self> {
        let normalized = contents.replace("\r\n", "\n");
        let Some(after_opening) = normalized.strip_prefix("---\n") else {
            bail!("template must start with YAML frontmatter");
        };
        let Some((frontmatter, preamble)) = after_opening.split_once("\n---\n")
        else {
            bail!("template YAML frontmatter must end with ---");
        };
        let frontmatter: Frontmatter = serde_yaml::from_str(frontmatter)
            .context("failed to parse template frontmatter")?;

        Ok(Self {
            model: frontmatter.model,
            preamble: preamble.trim().to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::AgentTemplate;

    #[test]
    fn parses_frontmatter_and_preamble() {
        let template = AgentTemplate::parse(
            "---\nmodel: gpt-5.3-codex\n---\nReturn JSON only.\n",
        )
        .unwrap();

        assert_eq!(template.model, "gpt-5.3-codex");
        assert_eq!(template.preamble, "Return JSON only.");
    }

    #[test]
    fn rejects_missing_frontmatter() {
        assert!(AgentTemplate::parse("Return JSON only.").is_err());
    }

    #[test]
    fn parses_windows_line_endings() {
        let template = AgentTemplate::parse(
            "---\r\nmodel: gpt-5.3-codex\r\n---\r\nReturn JSON only.\r\n",
        )
        .unwrap();

        assert_eq!(template.preamble, "Return JSON only.");
    }
}
