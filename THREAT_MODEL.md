# Threat Model — RVault

## Overview

RVault is a local, file-based secrets vault.
It encrypts all stored secrets using a key derived from a user-provided password.
Secrets are decrypted only in memory and only for the duration of an operation.

---

## Assets

- Secret values (API keys, tokens, passwords)
- Vault integrity
- Vault confidentiality

---

## Trust Assumptions

Trusted:
- RVault binary built from verified source
- Operating system kernel and filesystem permissions

Untrusted:
- Anyone with access to the vault file
- Backup systems and removable storage
- Accidental misuse by the user

Out of Scope:
- Malware or keyloggers
- Root or administrator-level attackers
- Live memory inspection attacks


## Security Goals

- Confidentiality of secrets at rest
- Integrity and authenticity of vault data
- Crash-safe and deterministic behavior
- Minimal plaintext exposure in memory

## Cryptographic Design

- Argon2 is used for password-based key derivation
- XChaCha20-Poly1305 provides authenticated encryption
- A fresh salt and nonce are generated for each encryption
- Salt and nonce are stored to enable decryption
