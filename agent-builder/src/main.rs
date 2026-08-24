mod config;
mod request;
mod template;

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
};

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
    let args = Args::parse();
    let config = Config::load(&args.config)?;
    let template = AgentTemplate::load(&config.template_path())?;
    let attachment = read_attachment(&args.file)?;
    let prompt = assemble_prompt(&args.request, &args.file, &attachment);
    let model = config.model.as_deref().unwrap_or(&template.model);

    let client = copilot::Client::from_env()?;
    let agent = client.agent(model).preamble(&template.preamble).build();
    let response = agent.prompt(&prompt).await?;

    println!("{}", response.trim());

    Ok(())
}
