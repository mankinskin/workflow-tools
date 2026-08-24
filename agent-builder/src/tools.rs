use std::{
    io,
    path::{
        Path,
        PathBuf,
    },
};

use anyhow::{
    Context,
    Result,
    bail,
};
use rmcp::{
    ServiceExt,
    transport::TokioChildProcess,
};
use serde::Deserialize;

use crate::{
    config::Config,
    template::AgentTemplate,
};

const READ_FILE: &str = "read_file";
const TICKET_LOOKUP: &str = "ticket_lookup";
const TICKET_MCP_TOOL: &str = "get_ticket";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct AuthorizedTools {
    pub read_file: bool,
    pub ticket_lookup: bool,
}

impl AuthorizedTools {
    pub fn from_template(template: &AgentTemplate) -> Result<Self> {
        let mut authorized = Self::default();
        for tool in &template.tools {
            match tool.as_str() {
                READ_FILE => authorized.read_file = true,
                TICKET_LOOKUP => authorized.ticket_lookup = true,
                unsupported => bail!(
                    "template authorizes unsupported tool `{unsupported}`"
                ),
            }
        }
        Ok(authorized)
    }

    #[cfg(test)]
    fn registered_names(&self) -> Vec<&'static str> {
        let mut names = Vec::new();
        if self.read_file {
            names.push(READ_FILE);
        }
        if self.ticket_lookup {
            names.push(TICKET_MCP_TOOL);
        }
        names
    }
}

pub struct ReadFileTool {
    root: PathBuf,
}

#[derive(Deserialize)]
pub struct ReadFileArgs {
    path: PathBuf,
}

impl ReadFileTool {
    pub fn new(root: &Path) -> io::Result<Self> {
        Ok(Self {
            root: root.canonicalize()?,
        })
    }

    fn read(
        &self,
        path: &Path,
    ) -> io::Result<String> {
        if path.is_absolute() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "read_file rejects absolute paths",
            ));
        }

        let resolved = self.root.join(path).canonicalize()?;
        if !resolved.starts_with(&self.root) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "read_file path escapes the configured root",
            ));
        }
        std::fs::read_to_string(resolved)
    }
}

impl rig::tool::Tool for ReadFileTool {
    const NAME: &'static str = READ_FILE;
    type Args = ReadFileArgs;
    type Output = String;
    type Error = io::Error;

    fn description(&self) -> String {
        "Read a UTF-8 file beneath the configured file root.".to_owned()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": { "path": { "type": "string" } },
            "required": ["path"],
            "additionalProperties": false,
        })
    }

    fn call(
        &self,
        _context: &mut rig::tool::ToolContext,
        args: Self::Args,
    ) -> impl std::future::Future<
        Output = std::result::Result<Self::Output, Self::Error>,
    > + Send {
        async move { self.read(&args.path) }
    }
}

pub struct TicketMcpConnection {
    pub service: rmcp::service::RunningService<rmcp::service::RoleClient, ()>,
    pub tool: rmcp::model::Tool,
}

impl TicketMcpConnection {
    pub async fn connect(config: &Config) -> Result<Self> {
        let mut command =
            tokio::process::Command::new(&config.ticket_mcp_command);
        command
            .args(&config.ticket_mcp_args)
            .current_dir(&config.ticket_workspace);
        let transport = TokioChildProcess::new(command)
            .context("failed to start configured ticket-mcp command")?;
        let service =
            ().serve(transport)
                .await
                .context("failed to initialize ticket-mcp")?;
        let tool = service
            .peer()
            .list_all_tools()
            .await
            .context("failed to list ticket-mcp tools")?
            .into_iter()
            .find(|tool| tool.name == TICKET_MCP_TOOL)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "ticket-mcp does not expose `{TICKET_MCP_TOOL}`"
                )
            })?;

        Ok(Self { service, tool })
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io,
        path::Path,
        time::{
            SystemTime,
            UNIX_EPOCH,
        },
    };

    use super::{
        AuthorizedTools,
        ReadFileTool,
    };
    use crate::template::AgentTemplate;

    #[test]
    fn absent_template_tool_is_not_registered() {
        let template = AgentTemplate::parse(
            "---\nmodel: gpt-5.3-codex\ntools:\n  - read_file\n---\nReturn JSON only.\n",
        )
        .unwrap();

        let authorized = AuthorizedTools::from_template(&template).unwrap();

        assert_eq!(authorized.registered_names(), ["read_file"]);
    }

    #[test]
    fn read_file_rejects_parent_path_escape() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("agent-builder-{unique}"));
        let nested = root.join("nested");
        fs::create_dir_all(&nested).unwrap();
        fs::write(root.parent().unwrap().join("outside.txt"), "outside")
            .unwrap();

        let tool = ReadFileTool::new(&root).unwrap();
        let error = tool.read(Path::new("../outside.txt")).unwrap_err();

        assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
        fs::remove_file(root.parent().unwrap().join("outside.txt")).unwrap();
        fs::remove_dir_all(root).unwrap();
    }
}
