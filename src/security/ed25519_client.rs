//! # ed25519_client
//!
//! A professional Rust module for signing gRPC requests and verifying gRPC responses
//! using ED25519 signatures, based on `ed25519-dalek` 1.0+, `tonic`, and `prost`.
//!
//! ## Features
//!
//! - Sign raw byte slices or complete Protobuf messages
//! - Attach signature and public key metadata to outgoing gRPC requests via a `tonic::Interceptor` (binary headers)
//! - Verify signature metadata on incoming gRPC responses
//! - Customizable metadata header keys
//! - Comprehensive error handling and testing

use ed25519_dalek::{Signature, SigningKey, VerifyingKey, Signer, Verifier};
use prost::Message;
use thiserror::Error;
use tonic::{Request, Response, Status, service::Interceptor};
use tonic::metadata::{MetadataKey, BinaryMetadataValue, Binary};
use std::convert::TryFrom;

/// Errors that can occur in the ed25519_client module
#[derive(Error, Debug)]
pub enum Ed25519ClientError {
    /// Protobuf encoding error
    #[error("Protobuf encoding error: {0}")]
    ProstEncode(#[from] prost::EncodeError),

    /// Signature or key parsing error
    #[error("Signature parsing error: {0}")]
    SignatureError(#[from] ed25519_dalek::SignatureError),

    /// gRPC status error
    #[error("gRPC status error: {0}")]
    TonicStatus(#[from] Status),
}

/// Sign a slice of bytes using the ED25519 signing key
pub fn sign_bytes(key: &SigningKey, data: &[u8]) -> Signature {
    key.sign(data)
}

/// Verify a slice of bytes against a signature and verifying key
pub fn verify_bytes(
    key: &VerifyingKey,
    data: &[u8],
    signature: &Signature,
) -> Result<(), ed25519_dalek::SignatureError> {
    key.verify(data, signature)
}

/// Sign a complete Protobuf message
pub fn sign_message<M: Message>(
    key: &SigningKey,
    message: &M,
) -> Result<Signature, prost::EncodeError> {
    let mut buf = Vec::with_capacity(message.encoded_len());
    message.encode(&mut buf)?;
    Ok(sign_bytes(key, &buf))
}

/// Verify a complete Protobuf message
pub fn verify_message<M: Message>(
    key: &VerifyingKey,
    message: &M,
    signature: &Signature,
) -> Result<(), ed25519_dalek::SignatureError> {
    let mut buf = Vec::with_capacity(message.encoded_len());
    message.encode(&mut buf).expect("Encoding failed");
    verify_bytes(key, &buf, signature)
}

/// Interceptor that signs outgoing gRPC requests with binary metadata
pub struct SigningInterceptor {
    signing_key: SigningKey,
    sig_header: MetadataKey<Binary>,
    pubkey_header: MetadataKey<Binary>,
}

impl SigningInterceptor {
    /// Create a new SigningInterceptor
    ///
    /// - `signing_key`: ED25519 signing key
    /// - `sig_header`: metadata key for the signature (Binary, e.g., "x-signature-bin")
    /// - `pubkey_header`: metadata key for the public key (Binary, e.g., "x-pubkey-bin")
    pub fn new(
        signing_key: SigningKey,
        sig_header: impl Into<MetadataKey<Binary>>,
        pubkey_header: impl Into<MetadataKey<Binary>>,
    ) -> Self {
        Self {
            signing_key,
            sig_header: sig_header.into(),
            pubkey_header: pubkey_header.into(),
        }
    }
}

impl Interceptor for SigningInterceptor {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        // Serialize request
        let mut buf = Vec::new();
        req.get_ref()
            .encode(&mut buf)
            .map_err(|e| Status::internal(format!("Encode error: {}", e)))?;

        // Generate signature and pubkey bytes
        let sig_bytes = self.signing_key.sign(&buf).to_bytes();
        let pubkey_bytes = self.signing_key.verifying_key().to_bytes();

        // Insert binary metadata
        req.metadata_mut().insert_bin(
            self.sig_header.clone(),
            BinaryMetadataValue::from_bytes(&sig_bytes),
        );
        req.metadata_mut().insert_bin(
            self.pubkey_header.clone(),
            BinaryMetadataValue::from_bytes(&pubkey_bytes),
        );

        Ok(req)
    }
}

/// Verifier for incoming gRPC responses with binary metadata
pub struct ResponseVerifier {
    server_pubkey: VerifyingKey,
    sig_header: MetadataKey<Binary>,
}

impl ResponseVerifier {
    /// Create a new ResponseVerifier
    ///
    /// - `server_pubkey`: the known server public key
    /// - `sig_header`: metadata key for the signature header (Binary, e.g., "x-signature-bin")
    pub fn new(
        server_pubkey: VerifyingKey,
        sig_header: impl Into<MetadataKey<Binary>>,
    ) -> Self {
        Self { server_pubkey, sig_header: sig_header.into() }
    }

    /// Verify the signature on a gRPC response
    pub fn verify<M: Message>(
        &self,
        response: &Response<M>,
    ) -> Result<(), Ed25519ClientError> {
        // Extract binary signature metadata
        let bytes = response
            .metadata()
            .get_bin(&self.sig_header)
            .ok_or_else(|| Status::unauthenticated("Missing signature header"))?;
        let signature = Signature::try_from(bytes.as_ref())
            .map_err(|_| Status::unauthenticated("Invalid signature"))?;

        // Serialize response message
        let mut buf = Vec::with_capacity(response.get_ref().encoded_len());
        response.get_ref()
            .encode(&mut buf)
            .map_err(Ed25519ClientError::ProstEncode)?;

        // Verify signature
        self.server_pubkey
            .verify(&buf, &signature)
            .map_err(Ed25519ClientError::SignatureError)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;
    use tonic::Response;
    use rand::rngs::OsRng;

    #[derive(Message, Clone)]
    struct TestMsg {
        #[prost(string, tag = "1")]
        pub field: String,
    }

    #[tokio::test]
    async fn test_sign_and_verify_functions() {
        let mut rng = OsRng;
        let key = SigningKey::generate(&mut rng);
        let msg = TestMsg { field: "hello".into() };

        let sig = sign_message(&key, &msg).expect("sign message");
        verify_message(&key.verifying_key(), &msg, &sig).expect("verify message");
    }

    #[tokio::test]
    async fn test_interceptor_and_verifier() {
        let mut rng = OsRng;
        let client_key = SigningKey::generate(&mut rng);
        let server_key = SigningKey::generate(&mut rng);

        // Simulate gRPC response
        let msg = TestMsg { field: "data".into() };
        let mut response = Response::new(msg.clone());
        let sig = server_key.sign(&msg.field.as_bytes()).to_bytes().to_vec();
        response.metadata_mut().insert_bin("x-signature-bin".parse().unwrap(), sig);

        let verifier = ResponseVerifier::new(server_key.verifying_key(), "x-signature-bin");
        verifier.verify(&response).expect("verify response");
    }
}
