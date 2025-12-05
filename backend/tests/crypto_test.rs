//! Crypto module tests

use bridgex_backend::crypto::keys::*;

#[test]
fn test_keypair_generation() {
    let keypair = generate_keypair();
    assert_eq!(keypair.public_key.len(), 32);
    assert_eq!(keypair.private_key.len(), 32);
    assert_ne!(keypair.public_key, keypair.private_key);
}

#[test]
fn test_keypair_uniqueness() {
    let keypair1 = generate_keypair();
    let keypair2 = generate_keypair();
    
    assert_ne!(keypair1.public_key, keypair2.public_key);
    assert_ne!(keypair1.private_key, keypair2.private_key);
}

#[test]
fn test_ecdh_key_exchange_symmetry() {
    let alice = generate_keypair();
    let bob = generate_keypair();

    let alice_shared = derive_shared_secret(&alice.private_key, &bob.public_key);
    let bob_shared = derive_shared_secret(&bob.private_key, &alice.public_key);

    assert_eq!(alice_shared, bob_shared, "Shared secrets must match");
}

#[test]
fn test_ecdh_different_peers_different_secrets() {
    let alice = generate_keypair();
    let bob = generate_keypair();
    let charlie = generate_keypair();

    let alice_bob = derive_shared_secret(&alice.private_key, &bob.public_key);
    let alice_charlie = derive_shared_secret(&alice.private_key, &charlie.public_key);

    assert_ne!(alice_bob, alice_charlie, "Different peer pairs should produce different secrets");
}

#[test]
fn test_session_key_derivation_deterministic() {
    let shared_secret = [42u8; 32];
    let info = b"test-context";

    let key1 = derive_session_key(&shared_secret, info);
    let key2 = derive_session_key(&shared_secret, info);

    assert_eq!(key1, key2, "Session key derivation should be deterministic");
}

#[test]
fn test_session_key_different_contexts() {
    let shared_secret = [42u8; 32];

    let key1 = derive_session_key(&shared_secret, b"context-1");
    let key2 = derive_session_key(&shared_secret, b"context-2");

    assert_ne!(key1, key2, "Different contexts should produce different keys");
}

#[test]
fn test_session_key_length() {
    let shared_secret = [0u8; 32];
    let session_key = derive_session_key(&shared_secret, b"test");
    
    assert_eq!(session_key.len(), 32, "Session key should be 32 bytes for AES-256");
}
