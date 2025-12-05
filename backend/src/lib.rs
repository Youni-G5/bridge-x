//! BridgeX Backend Library
//!
//! Core functionality for P2P file transfer, device pairing, and secure communication.

pub mod crypto;
pub mod db;
pub mod qr;
pub mod server;
pub mod util;

/// Re-export commonly used types
pub use crypto::keys::{generate_keypair, KeyPair};
pub use db::Database;
pub use qr::{generate_pairing_qr, generate_qr_data_url, generate_qr_svg};
