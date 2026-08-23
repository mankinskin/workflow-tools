use rig::client::{AgentClientExt, CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::copilot;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create the client from the environment variable.
    // - COPILOT_API_KEY
    // - OPENAI_API_KEY
    let client = copilot::Client::from_env()?;
    // Build an agent: a model plus a system prompt (the "preamble").
    let agent = client.agent(copilot::GPT_5_3_CODEX)
        .preamble("You are a helpful assistant.")
        .build();

    // Send a prompt and await the model's reply.
    let response = agent.prompt("What is the Rust programming language?").await?;

    println!("{response}");

    Ok(())
}