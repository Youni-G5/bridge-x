//! Cryptographic key management
//!
//! Handles X25519 key generation, ECDH key exchange, and session key derivation

use rand::rngs::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};

/// Cryptographic keypair
#[derive(Debug, Clone)]
pub struct KeyPair {
    pub public_key: [u8; 32],
    pub private_key: [u8; 32],
}

/// Generate a new X25519 keypair
///
/// # Returns
/// A new `KeyPair` with randomly generated keys
///
/// # Example
/// ```
/// use bridgex_backend::crypto::keys::generate_keypair;
///
/// let keypair = generate_keypair();
/// assert_eq!(keypair.public_key.len(), 32);
/// assert_eq!(keypair.private_key.len(), 32);
/// ```
pub fn generate_keypair() -> KeyPair {
    let secret = StaticSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);

    KeyPair {
        public_key: *public.as_bytes(),
        private_key: secret.to_bytes(),
    }
}

/// Perform ECDH key exchange
///
/// Derives a shared secret from a private key and peer's public key
///
/// # Arguments
/// * `private_key` - Our private key (32 bytes)
/// * `peer_public_key` - Peer's public key (32 bytes)
///
/// # Returns
/// Shared secret (32 bytes)
pub fn derive_shared_secret(private_key: &[u8; 32], peer_public_key: &[u8; 32]) -> [u8; 32] {
    let secret = StaticSecret::from(*private_key);
    let peer_public = PublicKey::from(*peer_public_key);
    let shared = secret.diffie_hellman(&peer_public);
    *shared.as_bytes()
}

/// Derive session key from shared secret using HKDF
///
/// # Arguments
/// * `shared_secret` - Shared secret from ECDH
/// * `info` - Context information (optional)
///
/// # Returns
/// Session key (32 bytes) suitable for AES-256-GCM
pub fn derive_session_key(shared_secret: &[u8; 32], info: &[u8]) -> [u8; 32] {
    use hkdf::Hkdf;
    use sha2::Sha256;

    let hk = Hkdf::<Sha256>::new(None, shared_secret);
    let mut okm = [0u8; 32];
    hk.expand(info, &mut okm)
        .expect("32 bytes is a valid length for HKDF-SHA256");
    okm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = generate_keypair();
        assert_eq!(keypair.public_key.len(), 32);
        assert_eq!(keypair.private_key.len(), 32);

        // Keys should be different
        assert_ne!(keypair.public_key, keypair.private_key);
    }

    #[test]
    fn test_ecdh_key_exchange() {
        // Alice generates keypair
        let alice = generate_keypair();
        // Bob generates keypair
        let bob = generate_keypair();

        // Both derive the same shared secret
        let alice_shared = derive_shared_secret(&alice.private_key, &bob.public_key);
        let bob_shared = derive_shared_secret(&bob.private_key, &alice.public_key);

        // Shared secrets must match
        assert_eq!(alice_shared, bob_shared);
    }

    #[test]
    fn test_session_key_derivation() {
        let shared_secret = [0u8; 32];
        let info = b"bridgex-session";

        let session_key = derive_session_key(&shared_secret, info);
        assert_eq!(session_key.len(), 32);

        // Same inputs should produce same key
        let session_key2 = derive_session_key(&shared_secret, info);
        assert_eq!(session_key, session_key2);

        // Different info should produce different key
        let session_key3 = derive_session_key(&shared_secret, b"different-info");
        assert_ne!(session_key, session_key3);
    }
}
