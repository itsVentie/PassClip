# PassClip

A minimalist, high-security background clipboard manager written in Rust. It automatically intercepts high-entropy data (passwords, tokens, private keys) from the system clipboard, encrypts it immediately in RAM using XChaCha20-Poly1305, and wipes the system clipboard. Retrieval requires Passkey (FIDO2/WebAuthn) authentication.

## Features

- **Automated Isolation:** Monitors clipboard and uses Shannon entropy calculation to detect sensitive data.
- **Zero-Persistence RAM Vault:** Keys and payloads are kept exclusively in memory.
- **Memory Hardening:** Uses the `PassClip` crate to securely zero-out encryption keys and plaintext buffers upon drop.
- **Biometric/Hardware Lock:** Authentication via local WebAuthn loopback layer (TouchID, Windows Hello, YubiKey).
- **Ephemeral Exposure:** Restores secrets to the system clipboard for 10 seconds only, then purges them.

## Installation

### Prerequisites
- Rust toolchain (stable)
- Libclang (for WebAuthn binding generation, if applicable)

### Build
```bash
cargo build --release