use crate::model::VaultData;
use crate::{crypto, format, storage, ui};

pub fn load(password_prompt: &str) -> Result<(VaultData, Vec<u8>, Vec<u8>), String> {
    let path = storage::vault_path()?;
    let raw = storage::read_all(&path)?;
    let vf = format::decode(&raw)?;
    let _ = vf.version;

    let password = ui::prompt_password(password_prompt)?;
    let plaintext = crypto::decrypt(&password, &vf.salt, &vf.nonce, &vf.ciphertext)?;

    let data: VaultData =
        serde_json::from_slice(&plaintext).map_err(|_| "Vault data corrupted".to_string())?;

    Ok((data, vf.salt, vf.nonce))
}

pub fn save(password: &str, data: &VaultData) -> Result<(), String> {
    let path = storage::vault_path()?;
    let plaintext = serde_json::to_vec(data).map_err(|e| e.to_string())?;

    let (salt, nonce, ciphertext) = crypto::encrypt(password, &plaintext)?;
    let bytes = format::encode(&salt, &nonce, &ciphertext)?;

    storage::write_atomic(&path, &bytes)
}
