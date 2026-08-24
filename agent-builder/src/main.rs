mod config;
mod request;
mod template;
mod tools;

use std::path::PathBuf;

use clap::Parser;
use rig::{
    client::{
        AgentClientExt,
        ProviderClient,
    },
    completion::Prompt,
    providers::copilot,
};

use crate::{
    config::Config,
    request::{
        assemble_prompt,
        read_attachment,
    },
    template::AgentTemplate,
    tools::{
        AuthorizedTools,
        ReadFileTool,
        TicketMcpConnection,
    },
};

/// Total model calls allowed per run: the initial call plus tool follow-ups.
const MAX_TURNS: usize = 10;

/// Rig logs tool names and arguments at `rig=debug`; opt in via `RUST_LOG`.
fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .with_writer(std::io::stderr)
        .init();
}

#[derive(Debug, Parser)]
#[command(about = "Execute one configured agent request with an attached file")]
struct Args {
    /// The request sent to the configured agent.
    request: String,

    /// Path to the file included in the model prompt.
    #[arg(short, long)]
    file: PathBuf,

    /// Path to the TOML configuration file.
    #[arg(short, long)]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init_tracing();

    let args = Args::parse();
    let config = Config::load(&args.config)?;
    let template = AgentTemplate::load(&config.template_path())?;
    let attachment = read_attachment(&args.file)?;
    let prompt = assemble_prompt(&args.request, &args.file, &attachment);
    let model = config.model.as_deref().unwrap_or(&template.model);
    let tools = AuthorizedTools::from_template(&template)?;

    let ticket_mcp = if tools.ticket_lookup {
        Some(TicketMcpConnection::connect(&config).await?)
    } else {
        None
    };

    let mut tool_server = rig::tool::server::ToolServer::new();
    if tools.read_file {
        tool_server = tool_server.tool(ReadFileTool::new(&config.file_root)?);
    }
    if let Some(ticket_mcp) = &ticket_mcp {
        tool_server = tool_server.rmcp_tool(
            ticket_mcp.tool.clone(),
            ticket_mcp.service.peer().clone(),
        );
    }
    let tool_server = tool_server.run();

    let client = copilot::Client::from_env()?;
    let agent = client
        .agent(model)
        .preamble(&template.preamble)
        .tool_server_handle(tool_server)
        .build();
    let response = agent.prompt(&prompt).max_turns(MAX_TURNS).await?;

    println!("{}", response.trim());

    Ok(())
}
