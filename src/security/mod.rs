//! # Security Module
//!
//! The `security` module provides a variety of cryptographic utilities to
//! ensure data confidentiality, integrity, and authenticity across your application.
//!
//! ## Submodules
//!
//! - **aes_256_gcm**: Authenticated encryption using AES-256-GCM.
//! - **ed25519_client**: Signing gRPC requests and verifying gRPC responses via ED25519.
//!
//! ## Passphrase-Based Encryption
//! The AES-GCM submodule supports password-derived keys via PBKDF2-HMAC-SHA256.
//!
//! ## Protobuf & gRPC Authentication
//! The `ed25519_client` submodule enables message signing and verification in gRPC flows
//! by providing a `tonic::Interceptor` and response verification utilities.
//!
//! ## ECIES Re-export
//! For convenience, the module re-exports ECIES asymmetric encryption:
//!
//! ```rust
//! pub use ecies;
//! ```

pub mod aes_256_gcm;
pub mod ed25519_client;

pub use ecies;
