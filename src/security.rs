pub use ecies;

pub use ed25519_dalek as ed25519;

pub mod aes_256_gcm {
    use aes_gcm::{
        aead::{rand_core::RngCore, Aead, KeyInit, OsRng},
        AeadCore, Aes256Gcm, Key, Nonce,
    };
    use pbkdf2::pbkdf2_hmac_array;
    use sha2::Sha256;

    const PBKDF2_ITERATIONS: u32 = 100_000;
    const SALT_LENGTH: usize = 16;
    const NONCE_LENGTH: usize = 12;

    fn derive_key(password: &[u8], salt: &[u8]) -> [u8; 32] {
        pbkdf2_hmac_array::<Sha256, 32>(password, salt, PBKDF2_ITERATIONS)
    }

    pub fn generate_key() -> [u8; 32] {
        Aes256Gcm::generate_key(&mut OsRng).into()
    }

    pub fn encrypt(key_bytes: &[u8; 32], plaintext: &[u8]) -> anyhow::Result<Vec<u8>> {
        let key = Key::<Aes256Gcm>::from_slice(key_bytes);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref())?;
        let mut result = Vec::with_capacity(NONCE_LENGTH + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    pub fn decrypt(key_bytes: &[u8; 32], data: &[u8]) -> anyhow::Result<Vec<u8>> {
        if data.len() < NONCE_LENGTH {
            anyhow::bail!("Ciphertext too short");
        }
        let (nonce_bytes, ciphertext) = data.split_at(NONCE_LENGTH);
        let key = Key::<Aes256Gcm>::from_slice(key_bytes);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())?;
        Ok(plaintext)
    }

    pub fn encrypt_with_passphrase(passphrase: &[u8], plaintext: &[u8]) -> anyhow::Result<Vec<u8>> {
        let mut salt = [0u8; SALT_LENGTH];
        OsRng.fill_bytes(&mut salt);
        let key = derive_key(passphrase, &salt);
        let mut encrypted = encrypt(&key, plaintext)?;
        let mut result = Vec::with_capacity(SALT_LENGTH + encrypted.len());
        result.extend_from_slice(&salt);
        result.append(&mut encrypted);
        Ok(result)
    }

    pub fn decrypt_with_passphrase(passphrase: &[u8], data: &[u8]) -> anyhow::Result<Vec<u8>> {
        if data.len() < SALT_LENGTH + NONCE_LENGTH {
            anyhow::bail!("Ciphertext too short");
        }
        let (salt, rest) = data.split_at(SALT_LENGTH);
        let key = derive_key(passphrase, salt);
        decrypt(&key, rest)
    }
}

#[cfg(test)]
mod tests {
    use super::aes_256_gcm::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = generate_key();
        let message = b"Secret data!";
        let encrypted = encrypt(&key, message).expect("encryption failed");
        let decrypted = decrypt(&key, &encrypted).expect("decryption failed");
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_with_passphrase() {
        let pass = b"p@ssw0rd!";
        let message = b"Very private!";
        let encrypted = encrypt_with_passphrase(pass, message).expect("encryption failed");
        let decrypted = decrypt_with_passphrase(pass, &encrypted).expect("decryption failed");
        assert_eq!(decrypted, message);
    }
}
