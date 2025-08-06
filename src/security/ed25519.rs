use ring::rand::SystemRandom;
pub use ring::signature::KeyPair;
use ring::signature::{Ed25519KeyPair, UnparsedPublicKey};

pub fn generate() -> anyhow::Result<Vec<u8>> {
    let rng = SystemRandom::new();
    Ok(Ed25519KeyPair::generate_pkcs8(&rng)?.as_ref().to_vec())
}

pub fn keypair_from_bytes(key_bytes: &[u8]) -> anyhow::Result<Ed25519KeyPair> {
    Ok(Ed25519KeyPair::from_pkcs8(key_bytes)?)
}

pub fn verify(pubkey_bytes: &[u8], msg: &[u8], signature: &[u8]) -> anyhow::Result<()> {
    UnparsedPublicKey::new(&ring::signature::ED25519, pubkey_bytes)
        .verify(msg, signature)
        .map_err(|e| anyhow::anyhow!("signature verification failed: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ring::signature;

    /// Test that generate() produces valid PKCS#8 bytes and can be loaded.
    #[test]
    fn test_generate_and_parse() {
        let pkcs8 = generate().expect("Failed to generate PKCS#8 bytes");
        let kp = keypair_from_bytes(&pkcs8).expect("Failed to parse PKCS#8 bytes");
        // Ensure public key loads
        let pubkey = kp.public_key().as_ref();
        assert_eq!(pubkey.len(), 32);
    }

    /// Test signing and verifying with generated key pair.
    #[test]
    fn test_sign_verify_flow() {
        let pkcs8 = generate().unwrap();
        let kp = keypair_from_bytes(&pkcs8).unwrap();
        let msg = b"flow test";
        let sig = kp.sign(msg);
        assert!(
            signature::UnparsedPublicKey::new(&signature::ED25519, kp.public_key().as_ref())
                .verify(msg, sig.as_ref())
                .is_ok()
        );
    }

    /// Test the verify() helper for valid, tampered message, and tampered signature.
    #[test]
    fn test_verify_helper() {
        let pkcs8 = generate().unwrap();
        let kp = keypair_from_bytes(&pkcs8).unwrap();
        let msg = b"verify helper";
        let sig = kp.sign(msg);

        // Valid
        verify(kp.public_key().as_ref(), msg, sig.as_ref()).expect("Valid signature failed");

        // Tampered message
        let badmsg = b"bad message";
        assert!(verify(kp.public_key().as_ref(), badmsg, sig.as_ref()).is_err());

        // Tampered signature
        let mut badsig = sig.as_ref().to_vec();
        badsig[0] = !badsig[0];
        assert!(verify(kp.public_key().as_ref(), msg, &badsig).is_err());
    }
}
