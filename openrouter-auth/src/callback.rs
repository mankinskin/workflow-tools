use std::time::Duration;
use tiny_http::{Response, Server};

#[derive(Debug)]
pub struct CallbackError(pub String);

impl std::fmt::Display for CallbackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CallbackError {}

/// A one-shot, loopback-only HTTP callback listener used to receive the
/// OAuth authorization code redirect. Binds to an OS-assigned ephemeral port.
pub struct LoopbackCallback {
    server: Server,
    port: u16,
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
        Ok(Self { server, port })
    }

    pub fn callback_url(&self) -> String {
        format!("http://127.0.0.1:{}/callback", self.port)
    }

    /// Blocks until exactly one request carrying an authorization `code`
    /// query parameter arrives, or the timeout elapses. Responds to the
    /// browser and then stops listening; never logs the received code.
    pub fn wait_for_code(&self, timeout: Duration) -> Result<String, CallbackError> {
        let request = self
            .server
            .recv_timeout(timeout)
            .map_err(|err| CallbackError(format!("loopback listener error: {err}")))?
            .ok_or_else(|| {
                CallbackError("timed out waiting for the browser redirect".to_string())
            })?;

        let code = extract_query_param(request.url(), "code");

        let response_body = if code.is_some() {
            "Authentication complete. You may close this tab and return to the terminal."
        } else {
            "Authentication failed: no authorization code was received."
        };
        let _ = request.respond(Response::from_string(response_body));

        code.ok_or_else(|| {
            CallbackError("callback request did not include an authorization code".to_string())
        })
    }
}

fn extract_query_param(url: &str, key: &str) -> Option<String> {
    let query = url.split_once('?')?.1;
    query.split('&').find_map(|pair| {
        let (k, v) = pair.split_once('=')?;
        if k == key {
            Some(percent_decode(v))
        } else {
            None
        }
    })
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

        let handle = std::thread::spawn(move || {
            let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
            let request = "GET /callback?code=fake-auth-code&state=xyz HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
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
