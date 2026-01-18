use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct SecretEntry {
    pub value: String,
    pub created_at: u64,
}

pub type VaultData = HashMap<String, SecretEntry>;
