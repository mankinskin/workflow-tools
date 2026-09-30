mod callback;
mod cli;
mod credential;
mod exchange;
mod pkce;

use clap::Parser;
use cli::{Cli, Command};
use credential::{CredentialStore, KeyringStore};
use std::io::{self, Write};
use std::time::Duration;

const SERVICE_NAME: &str = "openrouter-auth";
const ACCOUNT_NAME: &str = "api-key";
const CALLBACK_TIMEOUT: Duration = Duration::from_secs(600);

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Login => run_login(),
        Command::Token => run_token(),
    };
    if let Err(message) = result {
        eprintln!("error: {message}");
        std::process::exit(1);
    }
}

fn run_login() -> Result<(), String> {
    let store = KeyringStore::new(SERVICE_NAME, ACCOUNT_NAME)
        .map_err(|err| format!("credential store unavailable: {err}"))?;

    let pkce = pkce::Pkce::generate();
    let callback = callback::LoopbackCallback::bind()
        .map_err(|err| format!("failed to start loopback callback: {err}"))?;
    let callback_url = callback.callback_url();

    let authorize_url = exchange::build_authorize_url(&callback_url, &pkce.challenge_s256());
    if let Err(error) = webbrowser::open(&authorize_url) {
        eprintln!(
            "Could not open the default browser ({error}); configure a desktop browser and retry."
        );
        return Err("browser launch failed".to_string());
    }
    eprintln!("Opened the OpenRouter authorization page in your browser.");

    let code = callback
        .wait_for_code(CALLBACK_TIMEOUT)
        .map_err(|err| format!("did not receive an authorization code: {err}"))?;

    let key = exchange::exchange_code(exchange::KEYS_EXCHANGE_URL, &code, pkce.verifier())
        .map_err(|err| format!("failed to exchange authorization code: {err}"))?;

    store
        .store_secret(&key)
        .map_err(|err| format!("failed to store the API key: {err}"))?;

    eprintln!("Stored the OpenRouter API key in the OS credential store.");
    Ok(())
}

fn run_token() -> Result<(), String> {
    let store = KeyringStore::new(SERVICE_NAME, ACCOUNT_NAME)
        .map_err(|err| format!("credential store unavailable: {err}"))?;
    let key = store.load_secret().map_err(|err| err.to_string())?;
    write_token(&mut io::stdout().lock(), &key)
        .map_err(|_| "failed to write token to stdout".to_string())
}

fn write_token(mut output: impl Write, token: &str) -> Result<(), io::Error> {
    writeln!(output, "{token}")
}

#[cfg(test)]
mod tests {
    use super::write_token;

    #[test]
    fn token_output_contains_only_the_token_and_one_newline() {
        let mut output = Vec::new();
        write_token(&mut output, "fake-test-token").unwrap();
        assert_eq!(output, b"fake-test-token\n");
    }
}
