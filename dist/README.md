# RVault — Secure Local Secrets Vault (No Cloud, No BS)

RVault is a small, auditable, CLI-first secrets vault written in Rust.
Secrets are stored encrypted at rest and are only decrypted in memory during an operation.



## Why RVault?

Most secret managers prioritize UX and automation.
RVault prioritizes understandability and control.

This project is intentionally:
- Local-only
- Explicit in behavior
- Small enough to audit
- Honest about its threat model


## Features

- Encrypted vault file (no plaintext on disk)
- Password-based key derivation
- Authenticated encryption
- Crash-safe atomic writes
- Versioned binary file format


## How It Works (High Level)

1. User provides a password
2. A cryptographic key is derives
3. Secrets are decrypted in memory
4. Operation is performed (add/get/list/remove)
5. Vault is re-encrypted and written atomically to disk

At no point are secrets stored unencrypted on disk.

## Vault File Layout

vault.bin

- Magic bytes: RVLT
- Version number
- Salt
- Nonce
- Ciphertext

---

## What RVault Protects Against

- Stolen or copied vault file
- Accidental disclosure via backups
- File tampering or corruption
- Crashes during write operations

## What RVault Does NOT Protect Against

- Malware, keyloggers, or compromised OS
- An attacker with access to process memory while unlocked
- Password recovery if the password is lost

See THREAT_MODEL.md for full details.


## Usage

```bash
cargo build --release
./target/release/vault init
./target/release/vault list
```

---

## Security Notes

- Salt and nonce are stored alongside ciphertext and are not secret
- Wrong password and corrupted vault are indistinguishable by design
- Password strength directly impacts security

---

## License

MIT
