use std::io::Read;

pub const MAGIC: [u8; 4] = *b"RVLT";
pub const VERSION: u16 = 1;

#[derive(Debug)]
pub struct VaultFile {
    pub version: u16,
    pub salt: Vec<u8>,
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
}

fn put_u16(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_le_bytes());
}
fn put_u32(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn get_u16(bytes: &mut &[u8]) -> Result<u16, String> {
    let mut b = [0u8; 2];
    bytes.read_exact(&mut b).map_err(|e| e.to_string())?;
    Ok(u16::from_le_bytes(b))
}
fn get_u32(bytes: &mut &[u8]) -> Result<u32, String> {
    let mut b = [0u8; 4];
    bytes.read_exact(&mut b).map_err(|e| e.to_string())?;
    Ok(u32::from_le_bytes(b))
}
fn get_vec(bytes: &mut &[u8], len: usize) -> Result<Vec<u8>, String> {
    let mut v = vec![0u8; len];
    bytes.read_exact(&mut v).map_err(|e| e.to_string())?;
    Ok(v)
}

pub fn encode(salt: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    if salt.len() > u16::MAX as usize {
        return Err("Salt too large".into());
    }
    if nonce.len() > u16::MAX as usize {
        return Err("Nonce too large".into());
    }
    if ciphertext.len() > u32::MAX as usize {
        return Err("Ciphertext too large".into());
    }

    let mut out = Vec::new();
    out.extend_from_slice(&MAGIC);
    put_u16(&mut out, VERSION);

    put_u16(&mut out, salt.len() as u16);
    out.extend_from_slice(salt);

    put_u16(&mut out, nonce.len() as u16);
    out.extend_from_slice(nonce);

    put_u32(&mut out, ciphertext.len() as u32);
    out.extend_from_slice(ciphertext);

    Ok(out)
}

pub fn decode(mut bytes: &[u8]) -> Result<VaultFile, String> {
    let mut magic = [0u8; 4];
    bytes.read_exact(&mut magic).map_err(|e| e.to_string())?;
    if magic != MAGIC {
        return Err("Invalid vault file (bad magic)".into());
    }

    let version = get_u16(&mut bytes)?;
    if version != VERSION {
        return Err(format!("Unsupported vault version: {}", version));
    }

    let salt_len = get_u16(&mut bytes)? as usize;
    let salt = get_vec(&mut bytes, salt_len)?;

    let nonce_len = get_u16(&mut bytes)? as usize;
    let nonce = get_vec(&mut bytes, nonce_len)?;

    let ct_len = get_u32(&mut bytes)? as usize;
    let ciphertext = get_vec(&mut bytes, ct_len)?;

    Ok(VaultFile {
        version,
        salt,
        nonce,
        ciphertext,
    })
}
