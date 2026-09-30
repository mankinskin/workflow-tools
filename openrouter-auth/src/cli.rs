use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "openrouter-auth",
    about = "OpenRouter OAuth PKCE credential CLI",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Authenticate via OpenRouter's browser PKCE flow and store the returned API key.
    Login,
    /// Print the stored OpenRouter API key to stdout.
    Token,
}
