//! BridgeX Backend Library
//!
//! Core functionality for P2P file transfer, device pairing, and secure communication.

pub mod crypto;
pub mod server;
pub mod util;

/// Re-export commonly used types
pub use crypto::keys::{generate_keypair, KeyPair};
