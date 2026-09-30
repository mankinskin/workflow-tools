use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngCore;
use sha2::{Digest, Sha256};

/// A PKCE (RFC 7636) code verifier and its derived S256 challenge.
pub struct Pkce {
    verifier: String,
}

impl Pkce {
    /// Generates a cryptographically random 32-byte verifier, base64url-encoded
    /// without padding (43 characters, within the RFC 7636 43-128 char bound).
    pub fn generate() -> Self {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self {
            verifier: URL_SAFE_NO_PAD.encode(bytes),
        }
    }

    pub fn verifier(&self) -> &str {
        &self.verifier
    }

    /// Derives the S256 code challenge: BASE64URL(SHA256(verifier)).
    pub fn challenge_s256(&self) -> String {
        let digest = Sha256::digest(self.verifier.as_bytes());
        URL_SAFE_NO_PAD.encode(digest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifier_has_rfc7636_length_and_charset() {
        let pkce = Pkce::generate();
        let verifier = pkce.verifier();
        assert!(verifier.len() >= 43 && verifier.len() <= 128);
        assert!(
            verifier
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        );
    }

    #[test]
    fn challenge_is_deterministic_for_a_given_verifier() {
        let pkce = Pkce::generate();
        assert_eq!(pkce.challenge_s256(), pkce.challenge_s256());
    }

    #[test]
    fn two_generated_verifiers_differ() {
        let a = Pkce::generate();
        let b = Pkce::generate();
        assert_ne!(a.verifier(), b.verifier());
    }
}
