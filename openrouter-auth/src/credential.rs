use keyring::Entry;

#[derive(Debug)]
pub struct CredentialError(pub String);

impl std::fmt::Display for CredentialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CredentialError {}

/// Abstraction over OS-native secure credential storage so callers (and
/// tests) can substitute an in-memory fake instead of a real keyring.
pub trait CredentialStore {
    fn store_secret(&self, secret: &str) -> Result<(), CredentialError>;
    fn load_secret(&self) -> Result<String, CredentialError>;
}

/// Stores the credential exclusively in the OS-native secure credential
/// store (Keychain on macOS, Credential Manager on Windows, Secret Service
/// on Linux). Fails closed: there is no plaintext or file-based fallback.
pub struct KeyringStore {
    entry: Entry,
}

impl KeyringStore {
    pub fn new(service: &str, account: &str) -> Result<Self, CredentialError> {
        let entry = Entry::new(service, account)
            .map_err(|err| CredentialError(format!("OS credential store unavailable: {err}")))?;
        Ok(Self { entry })
    }
}

impl CredentialStore for KeyringStore {
    fn store_secret(&self, secret: &str) -> Result<(), CredentialError> {
        self.entry.set_password(secret).map_err(|err| {
            CredentialError(format!("failed to write to OS credential store: {err}"))
        })
    }

    fn load_secret(&self) -> Result<String, CredentialError> {
        self.entry.get_password().map_err(|err| match err {
            keyring::Error::NoEntry => CredentialError(
                "no OpenRouter API key is stored; run `openrouter-auth login` first".to_string(),
            ),
            other => CredentialError(format!("failed to read from OS credential store: {other}")),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[derive(Default)]
    struct FakeCredentialStore {
        secret: Mutex<Option<String>>,
    }

    impl CredentialStore for FakeCredentialStore {
        fn store_secret(&self, secret: &str) -> Result<(), CredentialError> {
            *self.secret.lock().unwrap() = Some(secret.to_string());
            Ok(())
        }

        fn load_secret(&self) -> Result<String, CredentialError> {
            self.secret
                .lock()
                .unwrap()
                .clone()
                .ok_or_else(|| CredentialError("no secret stored".to_string()))
        }
    }

    #[test]
    fn fake_store_roundtrips_a_secret() {
        let store = FakeCredentialStore::default();
        store.store_secret("fake-test-credential").unwrap();
        assert_eq!(store.load_secret().unwrap(), "fake-test-credential");
    }

    #[test]
    fn fake_store_fails_closed_when_empty() {
        let store = FakeCredentialStore::default();
        assert!(store.load_secret().is_err());
    }
}
