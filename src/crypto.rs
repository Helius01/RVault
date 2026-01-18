use argon2::Argon2;
use chacha20poly1305::aead::rand_core::RngCore;
use chacha20poly1305::KeyInit;
use chacha20poly1305::{
    aead::{Aead, OsRng},
    Key, XChaCha20Poly1305, XNonce,
};
use zeroize::Zeroize;

pub struct DerivedKey {
    key: Key,
}

impl Drop for DerivedKey {
    fn drop(&mut self) {
        self.key.zeroize();
    }
}

pub fn derive_key(password: &str, salt: &[u8]) -> Result<DerivedKey, String> {
    let argon = Argon2::default();
    let mut key_bytes = [0u8; 32];

    argon
        .hash_password_into(password.as_bytes(), salt, &mut key_bytes)
        .map_err(|e| e.to_string())?;

    Ok(DerivedKey {
        key: Key::from_slice(&key_bytes).clone(),
    })
}

/// Encrypts plaintext using password.
/// Returns (salt16, nonce24, ciphertext)
pub fn encrypt(password: &str, plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), String> {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let dk = derive_key(password, &salt)?;
    let cipher = XChaCha20Poly1305::new(&dk.key);

    let mut nonce_bytes = [0u8; 24];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = XNonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| "Encryption failed".to_string())?;

    Ok((salt.to_vec(), nonce_bytes.to_vec(), ciphertext))
}

pub fn decrypt(
    password: &str,
    salt: &[u8],
    nonce: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, String> {
    let dk = derive_key(password, salt)?;
    let cipher = XChaCha20Poly1305::new(&dk.key);

    cipher
        .decrypt(XNonce::from_slice(nonce), ciphertext)
        .map_err(|_| "Invalid password or corrupted vault".to_string())
}
