# PassClip

A minimalist, high-security background clipboard manager written in Rust. It automatically intercepts high-entropy data (passwords, tokens, private keys) from the system clipboard, encrypts it immediately in RAM using XChaCha20-Poly1305, and wipes the system clipboard. Retrieval requires Passkey (FIDO2/WebAuthn) authentication.

## Features

- **Client-Server Model:** Runs as a lightweight background daemon with an asynchronous cross-platform IPC bridge (Named Pipes on Windows, Unix Domain Sockets on Linux/macOS).
- **Automated Isolation:** Monitors the system clipboard using a dedicated native thread and triggers Shannon entropy calculations to intercept sensitive leaks.
- **Zero-Persistence RAM Vault:** Keys and payloads are kept exclusively in memory with no disk footprint.
- **Memory Hardening:** Implements explicit memory zeroization (`zeroize` crate) for transient buffers and keys upon drop.
- **Biometric/Hardware Lock:** Authentication via FIDO2/WebAuthn infrastructure (Windows Hello, TouchID, YubiKey).
- **Ephemeral Exposure:** Safely returns secrets to the clipboard with automated lifecycle management.

## Installation

### Prerequisites
- Rust toolchain (stable)
- Libclang (for WebAuthn binding generation, if applicable)

### Build
```bash
cargo build --release