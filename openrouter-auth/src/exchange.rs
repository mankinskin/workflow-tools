use serde::Deserialize;
use std::time::Duration;

const EXCHANGE_TIMEOUT: Duration = Duration::from_secs(30);

pub const AUTHORIZE_URL: &str = "https://openrouter.ai/auth";
pub const KEYS_EXCHANGE_URL: &str = "https://openrouter.ai/api/v1/auth/keys";

#[derive(Debug)]
pub struct ExchangeError(pub String);

impl std::fmt::Display for ExchangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ExchangeError {}

/// Builds the browser-facing OpenRouter PKCE authorize URL for a
/// runtime-bound loopback callback. No `client_id` is included; none is
/// documented by OpenRouter for this flow.
pub fn build_authorize_url(callback_url: &str, code_challenge: &str) -> String {
    let mut url = url::Url::parse(AUTHORIZE_URL).expect("AUTHORIZE_URL is a valid URL constant");
    url.query_pairs_mut()
        .append_pair("callback_url", callback_url)
        .append_pair("code_challenge", code_challenge)
        .append_pair("code_challenge_method", "S256");
    url.into()
}

#[derive(Deserialize)]
struct KeysResponse {
    key: String,
}

/// Exchanges an authorization code for an OpenRouter API key. `exchange_url`
/// is parameterized so tests can target a local mock server instead of the
/// real OpenRouter endpoint; production callers pass [`KEYS_EXCHANGE_URL`].
pub fn exchange_code(
    exchange_url: &str,
    code: &str,
    code_verifier: &str,
) -> Result<String, ExchangeError> {
    let body = serde_json::json!({
        "code": code,
        "code_verifier": code_verifier,
        "code_challenge_method": "S256",
    });

    let client = reqwest::blocking::Client::builder()
        .timeout(EXCHANGE_TIMEOUT)
        .build()
        .map_err(|err| ExchangeError(format!("failed to initialize HTTP client: {err}")))?;
    let response = client
        .post(exchange_url)
        .json(&body)
        .send()
        .map_err(|err| ExchangeError(format!("OpenRouter exchange request failed: {err}")))?;

    if !response.status().is_success() {
        return Err(ExchangeError(format!(
            "OpenRouter rejected the exchange with status {}",
            response.status()
        )));
    }

    let parsed: KeysResponse = response
        .json()
        .map_err(|err| ExchangeError(format!("could not parse OpenRouter response: {err}")))?;
    Ok(parsed.key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;

    #[test]
    fn authorize_url_carries_pkce_parameters() {
        let url = build_authorize_url("http://127.0.0.1:41000/callback", "challenge-value");
        assert!(url.starts_with(AUTHORIZE_URL));
        assert!(url.contains("code_challenge=challenge-value"));
        assert!(url.contains("code_challenge_method=S256"));
        assert!(url.contains("callback_url="));
    }

    #[test]
    fn exchange_code_parses_a_mocked_success_response() {
        let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
        let addr = server.server_addr().to_ip().unwrap();
        let handle = std::thread::spawn(move || {
            let mut request = server.recv().unwrap();
            assert_eq!(request.method(), &tiny_http::Method::Post);
            assert_eq!(request.url(), "/api/v1/auth/keys");

            let mut request_body = String::new();
            request
                .as_reader()
                .read_to_string(&mut request_body)
                .unwrap();
            let request_json: serde_json::Value = serde_json::from_str(&request_body).unwrap();
            assert_eq!(request_json["code"], "fake-code");
            assert_eq!(request_json["code_verifier"], "fake-verifier");
            assert_eq!(request_json["code_challenge_method"], "S256");

            let response = tiny_http::Response::from_string("{\"key\":\"fake-test-key\"}")
                .with_header(
                    tiny_http::Header::from_bytes("Content-Type", "application/json").unwrap(),
                );
            request.respond(response).unwrap();
        });

        let url = format!("http://{addr}/api/v1/auth/keys");
        let key = exchange_code(&url, "fake-code", "fake-verifier").unwrap();
        assert_eq!(key, "fake-test-key");
        handle.join().unwrap();
    }

    #[test]
    fn exchange_code_fails_on_non_success_status() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0u8; 1024];
            let _ = stream.read(&mut buf).unwrap();
            let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        });

        let url = format!("http://{addr}/api/v1/auth/keys");
        let result = exchange_code(&url, "fake-code", "fake-verifier");
        assert!(result.is_err());
        handle.join().unwrap();
    }
}
