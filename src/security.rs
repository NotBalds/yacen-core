pub use ecies;

pub use ed25519_dalek as ed25519;

pub mod aes_256_gcm {
    use aes_gcm::{
        aead::{rand_core::RngCore, Aead, KeyInit, OsRng},
        AeadCore, Aes256Gcm, Key, Nonce,
    };
    use pbkdf2::pbkdf2_hmac_array;
    use sha2::Sha256;

    /// Derives a 256-bit AES key from a password and salt using PBKDF2-HMAC-SHA256.
    pub fn derive_key(password: &[u8], salt: &[u8], iterations: u32) -> [u8; 32] {
        pbkdf2_hmac_array::<Sha256, 32>(password, salt, iterations)
    }

    pub fn generate_key() -> [u8; 32] {
        Aes256Gcm::generate_key(&mut OsRng).into()
    }

    pub fn encrypt(key_bytes: &[u8; 32], plaintext: &[u8]) -> anyhow::Result<(Vec<u8>, [u8; 12])> {
        let key = Key::<Aes256Gcm>::from_slice(key_bytes);
        let cipher = Aes256Gcm::new(key);

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref())?;

        Ok((ciphertext, nonce.into()))
    }

    pub fn decrypt(
        key_bytes: &[u8; 32],
        nonce_bytes: &[u8; 12],
        ciphertext: &[u8],
    ) -> anyhow::Result<Vec<u8>> {
        let key = Key::<Aes256Gcm>::from_slice(key_bytes);
        let cipher = Aes256Gcm::new(key);

        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())?;

        Ok(plaintext)
    }

    pub fn encrypt_with_passphrase(
        passphrase: &[u8],
        plaintext: &[u8],
        iterations: u32,
    ) -> anyhow::Result<([u8; 16], Vec<u8>, [u8; 12])> {
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);
        let key = derive_key(passphrase, &salt, iterations);

        let (ciphertext, nonce) = encrypt(&key, plaintext)?;
        Ok((salt, ciphertext, nonce))
    }

    pub fn decrypt_with_passphrase(
        passphrase: &[u8],
        salt: &[u8; 16],
        nonce: &[u8; 12],
        ciphertext: &[u8],
        iterations: u32,
    ) -> anyhow::Result<Vec<u8>> {
        let key = derive_key(passphrase, salt, iterations);
        decrypt(&key, nonce, ciphertext)
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::aes_256_gcm;

    #[test]
    fn basic_roundtrip() {
        let key = aes_256_gcm::generate_key();
        let data = b"Hello, AES-GCM!";

        let (ciphertext, nonce) = aes_256_gcm::encrypt(&key, data).unwrap();
        let plaintext = aes_256_gcm::decrypt(&key, &nonce, &ciphertext).unwrap();

        assert_eq!(plaintext, data);
    }

    #[test]
    fn pbkdf2_roundtrip_with_passphrase() {
        let pass = b"s3cr3t";
        let data = b"Top secret data";
        let iter = 100_000;

        let (salt, ciphertext, nonce) =
            aes_256_gcm::encrypt_with_passphrase(pass, data, iter).unwrap();
        let plaintext =
            aes_256_gcm::decrypt_with_passphrase(pass, &salt, &nonce, &ciphertext, iter).unwrap();

        assert_eq!(plaintext, data);
    }
}
