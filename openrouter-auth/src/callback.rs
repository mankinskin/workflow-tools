use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngCore;
use std::time::Duration;
use tiny_http::{Method, Response, Server};

#[derive(Debug)]
pub struct CallbackError(pub String);

impl std::fmt::Display for CallbackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CallbackError {}

/// A one-shot, loopback-only HTTP callback listener used to receive the
/// OAuth authorization code redirect. Binds to an OS-assigned ephemeral port
/// and a fresh unguessable callback path for each login.
pub struct LoopbackCallback {
    server: Server,
    port: u16,
    callback_path: String,
}

impl LoopbackCallback {
    pub fn bind() -> Result<Self, CallbackError> {
        let server = Server::http("127.0.0.1:0")
            .map_err(|err| CallbackError(format!("failed to bind loopback listener: {err}")))?;
        let port = server
            .server_addr()
            .to_ip()
            .ok_or_else(|| {
                CallbackError("loopback listener did not return an IP address".to_string())
            })?
            .port();
        let mut nonce = [0_u8; 24];
        rand::thread_rng().fill_bytes(&mut nonce);
        let callback_path = format!("/callback/{}", URL_SAFE_NO_PAD.encode(nonce));
        Ok(Self {
            server,
            port,
            callback_path,
        })
    }

    pub fn callback_url(&self) -> String {
        format!("http://127.0.0.1:{}{}", self.port, self.callback_path)
    }

    /// Blocks until a single GET request reaches this login's unique callback
    /// path with exactly one non-empty authorization `code`, or the timeout elapses.
    /// Responds to the browser and then stops listening; never logs the code.
    pub fn wait_for_code(&self, timeout: Duration) -> Result<String, CallbackError> {
        let request = self
            .server
            .recv_timeout(timeout)
            .map_err(|err| CallbackError(format!("loopback listener error: {err}")))?
            .ok_or_else(|| {
                CallbackError("timed out waiting for the browser redirect".to_string())
            })?;

        let request_path = request.url().split('?').next().unwrap_or_default();
        let code = if request.method() == &Method::Get && request_path == self.callback_path {
            extract_single_query_param(request.url(), "code")
        } else {
            None
        };

        let response_body = if code.is_some() {
            "Authentication complete. You may close this tab and return to the terminal."
        } else {
            "Authentication failed: this callback is invalid or missing its authorization code."
        };
        let _ = request.respond(Response::from_string(response_body));

        code.ok_or_else(|| {
            CallbackError(
                "callback request was invalid or did not include one authorization code"
                    .to_string(),
            )
        })
    }
}

fn extract_single_query_param(url: &str, key: &str) -> Option<String> {
    let query = url.split_once('?')?.1;
    let mut matches = query.split('&').filter_map(|pair| {
        let (candidate, value) = pair.split_once('=')?;
        (candidate == key).then(|| percent_decode(value))
    });
    let value = matches.next()?;
    if value.is_empty() || matches.next().is_some() {
        return None;
    }
    Some(value)
}

fn percent_decode(value: &str) -> String {
    let mut bytes = Vec::with_capacity(value.len());
    let mut iter = value.bytes();
    while let Some(b) = iter.next() {
        match b {
            b'%' => {
                let hi = iter.next();
                let lo = iter.next();
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    let hex = [hi, lo];
                    if let Ok(hex_str) = std::str::from_utf8(&hex)
                        && let Ok(byte) = u8::from_str_radix(hex_str, 16)
                    {
                        bytes.push(byte);
                        continue;
                    }
                }
                bytes.push(b);
            }
            b'+' => bytes.push(b' '),
            other => bytes.push(other),
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpStream;

    #[test]
    fn wait_for_code_extracts_code_from_a_single_redirect() {
        let callback = LoopbackCallback::bind().unwrap();
        let port = callback.port;
        let callback_path = callback.callback_path.clone();

        let handle = std::thread::spawn(move || {
            let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
            let request = format!(
                "GET {callback_path}?code=fake-auth-code&state=xyz HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n"
            );
            stream.write_all(request.as_bytes()).unwrap();
            let mut buf = Vec::new();
            let _ = stream.read_to_end(&mut buf);
        });

        let code = callback
            .wait_for_code(Duration::from_secs(5))
            .expect("should receive code");
        assert_eq!(code, "fake-auth-code");
        handle.join().unwrap();
    }

    #[test]
    fn wait_for_code_rejects_a_request_to_a_different_path() {
        let callback = LoopbackCallback::bind().unwrap();
        let port = callback.port;

        let handle = std::thread::spawn(move || {
            let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
            let request = "GET /callback?code=fake-auth-code HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
            stream.write_all(request.as_bytes()).unwrap();
            let mut buf = Vec::new();
            let _ = stream.read_to_end(&mut buf);
        });

        let result = callback.wait_for_code(Duration::from_secs(5));
        assert!(result.is_err());
        handle.join().unwrap();
    }

    #[test]
    fn wait_for_code_rejects_duplicate_code_parameters() {
        let callback = LoopbackCallback::bind().unwrap();
        let port = callback.port;
        let callback_path = callback.callback_path.clone();

        let handle = std::thread::spawn(move || {
            let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
            let request = format!(
                "GET {callback_path}?code=first&code=second HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n"
            );
            stream.write_all(request.as_bytes()).unwrap();
            let mut buf = Vec::new();
            let _ = stream.read_to_end(&mut buf);
        });

        let result = callback.wait_for_code(Duration::from_secs(5));
        assert!(result.is_err());
        handle.join().unwrap();
    }

    #[test]
    fn wait_for_code_times_out_when_nothing_arrives() {
        let callback = LoopbackCallback::bind().unwrap();
        let result = callback.wait_for_code(Duration::from_millis(50));
        assert!(result.is_err());
    }

    #[test]
    fn callback_url_is_loopback_only() {
        let callback = LoopbackCallback::bind().unwrap();
        assert!(callback.callback_url().starts_with("http://127.0.0.1:"));
    }
}
