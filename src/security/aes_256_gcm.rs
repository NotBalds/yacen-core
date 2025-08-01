use aes_gcm::{
    aead::{rand_core::RngCore, Aead, KeyInit, OsRng},
    AeadCore, Aes256Gcm, Key, Nonce,
};
use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha256;

/// Number of iterations for PBKDF2 when deriving a key from a passphrase.
const PBKDF2_ITERATIONS: u32 = 100_000;
/// Length of the random salt in bytes.
const SALT_LENGTH: usize = 16;
/// Length of the GCM nonce in bytes.
const NONCE_LENGTH: usize = 12;

#[doc(hidden)]
/// Derive a 256-bit key from a passphrase and salt using PBKDF2-HMAC-SHA256.
fn derive_key(password: &[u8], salt: &[u8]) -> [u8; 32] {
    pbkdf2_hmac_array::<Sha256, 32>(password, salt, PBKDF2_ITERATIONS)
}

/// Generate a random 256-bit key suitable for AES-256-GCM.
///
/// # Examples
///
/// ```rust
/// use yacen_core::security::aes_256_gcm::generate_key;
/// let key = generate_key();
/// assert_eq!(key.len(), 32);
/// ```
pub fn generate_key() -> [u8; 32] {
    Aes256Gcm::generate_key(&mut OsRng).into()
}

/// Encrypt plaintext with a raw 256-bit key using AES-256-GCM.
///
/// Returns a byte vector containing [nonce || ciphertext || tag].
///
/// # Errors
/// Fails if encryption fails.
///
/// # Examples
///
/// ```rust
/// use yacen_core::security::aes_256_gcm::{generate_key, encrypt, decrypt};
///
/// let key = generate_key();
/// let plaintext = b"Hello, world!";
/// let encrypted = encrypt(&key, plaintext).expect("encryption failed");
/// let decrypted = decrypt(&key, &encrypted).expect("decryption failed");
/// assert_eq!(&decrypted, plaintext);
/// ```
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

/// Decrypt ciphertext produced by [`encrypt`] with the same key.
///
/// Expects input as [nonce || ciphertext || tag].
///
/// # Errors
/// Fails if input is too short or authentication fails.
///
/// # Examples
///
/// ```rust
/// use yacen_core::security::aes_256_gcm::{generate_key, encrypt, decrypt};
///
/// let key = generate_key();
/// let ciphertext = encrypt(&key, b"data").unwrap();
/// let plaintext = decrypt(&key, &ciphertext).unwrap();
/// assert_eq!(&plaintext, b"data");
/// ```
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

/// Encrypt plaintext with a passphrase.
///
/// A random 128-bit salt is generated and prepended to the output. The format is:
/// [salt || nonce || ciphertext || tag].
///
/// # Errors
/// Fails if encryption fails.
///
/// # Examples
///
/// ```rust
/// use yacen_core::security::aes_256_gcm::{encrypt_with_passphrase, decrypt_with_passphrase};
///
/// let pass = b"my secret";
/// let data = b"Top secret data";
/// let encrypted = encrypt_with_passphrase(pass, data).expect("encrypt failed");
/// let decrypted = decrypt_with_passphrase(pass, &encrypted).expect("decrypt failed");
/// assert_eq!(&decrypted, data);
/// ```
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

/// Decrypt data encrypted by [`encrypt_with_passphrase`].
///
/// Extracts the salt, then derives the key and decrypts the remainder.
///
/// # Errors
/// Fails if input is too short or authentication fails.
///
/// # Examples
///
/// ```rust
/// use yacen_core::security::aes_256_gcm::{encrypt_with_passphrase, decrypt_with_passphrase};
///
/// let pass = b"another secret";
/// let ciphertext = encrypt_with_passphrase(pass, b"hello").unwrap();
/// let plaintext = decrypt_with_passphrase(pass, &ciphertext).unwrap();
/// assert_eq!(&plaintext, b"hello");
/// ```
pub fn decrypt_with_passphrase(passphrase: &[u8], data: &[u8]) -> anyhow::Result<Vec<u8>> {
    if data.len() < SALT_LENGTH + NONCE_LENGTH {
        anyhow::bail!("Ciphertext too short");
    }
    let (salt, rest) = data.split_at(SALT_LENGTH);
    let key = derive_key(passphrase, salt);
    decrypt(&key, rest)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_decrypt_wrong_key_fails() {
        let key1 = generate_key();
        let key2 = generate_key();
        let data = b"Sensitive content";
        let encrypted = encrypt(&key1, data).expect("encryption failed");
        let result = decrypt(&key2, &encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_wrong_passphrase_fails() {
        let pass1 = b"correct-horse";
        let pass2 = b"battery-staple";
        let data = b"Private message";
        let encrypted = encrypt_with_passphrase(pass1, data).expect("encryption failed");
        let result = decrypt_with_passphrase(pass2, &encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_invalid_data_length() {
        let key = generate_key();
        let result = decrypt(&key, b"short");
        assert!(result.is_err());

        let pass = b"passphrase";
        let result = decrypt_with_passphrase(pass, b"short");
        assert!(result.is_err());
    }
}
