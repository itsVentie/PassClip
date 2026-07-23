# PassClip

A minimalist, high-security background clipboard manager written in Rust. It automatically intercepts high-entropy sensitive data (passwords, API tokens, private keys) from the system clipboard, encrypts it immediately in RAM using XChaCha20-Poly1305, and wipes the OS clipboard. Retrieval requires biometric or hardware-backed (FIDO2/WebAuthn / Windows Hello) authentication.

---

## Key Features

- **Client-Server Architecture:** Runs as an isolated background daemon with an asynchronous cross-platform IPC bridge (*Named Pipes* on Windows, *Unix Domain Sockets* on Linux/macOS).
- **Automated Secret Isolation:** Continuous clipboard monitoring calculates Shannon entropy thresholds to catch sensitive plaintexts automatically.
- **Zero-Persistence RAM Vault:** Secrets and encryption keys are held exclusively in volatile memory (`sync::Arc<Mutex<...>>`). Nothing touches the disk.
- **Memory Hardening:** Explicit memory zeroization (`zeroize` crate) guarantees that transient buffers and cryptographic keys are wiped upon `Drop`.
- **Hardware/Biometric Lock:** Integrated OS consent verification (Windows Hello, TouchID) and FIDO2/WebAuthn authentication stack.
- **Ephemeral Clipboard Exposure:** Restores secrets to the clipboard on demand and automatically wipes/restores previous clipboard contents after a 30-second timer.
- **Desktop Notifications:** Native OS toasts alerting on isolation, retrieval, and auto-wipe lifecycle events.

---

## Architecture & Data Flow


```

┌─────────────────────────────────────────────────────────┐
│                   System Clipboard                      │
└─────────────┬─────────────────────────────▲─────────────┘
│ Monitors                    │ Ephemeral Restore
▼                             │ (30s auto-wipe)
┌──────────────────────────┐   IPC Bridge   │
│   PassClip Daemon        ├────────────────┤
│   - Shannon Entropy Check│  (Named Pipe / │
│   - XChaCha20-Poly1305   │  Unix Socket)  │
│   - In-Memory Vault      │                │
└─────────────▲────────────┘                │
│ Authentication              │
│ (Windows Hello / FIDO2)     │
┌─────────────┴────────────┐                │
│   PassClip CLI           ├────────────────┘
│   `passclip pop`         │ Requests Secret
└──────────────────────────┘

```

---

## Prerequisites

- **Rust Toolchain:** Stable release (1.75+)
- **OS Support:** Windows 10/11 (with Windows Hello enabled) or Linux/macOS.

---

## Installation & Build

Clone the repository and build the release binary:

```bash
git clone [https://github.com/itsVentie/PassClip.git](https://github.com/itsVentie/PassClip.git)
cd PassClip
cargo build --release

```

The compiled binary will be located at `./target/release/passclip` (`passclip.exe` on Windows).

> **System-Wide Installation (Optional):**
> To run `passclip` directly from any directory without specifying executable paths, install it into your Cargo bin path:
> ```bash
> cargo install --path .
> 
> ```
> 
> 

---

## Quick Start & Usage

*Note: If you haven't run `cargo install --path .`, replace `passclip` with `.\target\release\passclip.exe` (Windows) or `./target/release/passclip` (Linux/macOS) in the commands below.*

### 1. Start the Background Daemon

Run the daemon in your terminal or set it up as a startup background service:

```bash
# System-wide
passclip daemon

# Local build target (Windows / Linux)
.\target\release\passclip.exe daemon
./target/release/passclip daemon

```

### 2. Copy a Secret

Copy any high-entropy text to your clipboard (e.g., `dK9#mX2!vL8$pQ5N8xY`).

PassClip will automatically:

1. Intercept and encrypt the secret in RAM.
2. Wipe the system clipboard immediately.
3. Fire a native OS notification confirming isolation.

### 3. Check Vault Status

In a separate terminal, verify if a secret is currently secured in the RAM vault:

```bash
passclip status
# Or: .\target\release\passclip.exe status

```

### 4. Retrieve the Secret

When you need to paste your secret:

```bash
passclip pop
# Or: .\target\release\passclip.exe pop

```

This triggers the OS consent prompt (e.g., **Windows Hello** PIN/biometrics). Upon successful verification:

* The secret is restored to the clipboard.
* A 30-second timer starts, after which the clipboard is wiped again and restored to its prior state.

---

## Configuration

PassClip can be configured via a `config.toml` file placed in the working directory:

```toml
# Security thresholds
min_entropy = 3.5
min_length = 8

# WebAuthn Relying Party settings
rp_id = "localhost"
rp_name = "PassClip Vault"
rp_origin = "http://localhost"

# System settings
enable_notifications = true

```

---

## CLI Reference

| Command | Description |
| --- | --- |
| `passclip daemon` | Starts the background service, clipboard monitor, and IPC server. |
| `passclip status` | Queries the daemon to check if a secret is currently held in the vault. |
| `passclip pop` | Requests authentication challenge and restores the secret to the clipboard. |
| `passclip --help` | Prints the standard CLI help message and version info. |

---

## Security Model

* **No Disk I/O:** PassClip does not write credentials, keys, or logs to disk.
* **Process Isolation:** The daemon holds the master key and payload in zeroized memory. External processes cannot access secrets without passing the IPC challenge.
* **Volatile Lifespans:** Restored secrets exist in the active OS clipboard buffer for a maximum of 30 seconds before explicit zeroization.

---

## License

Distributed under the Apache-2.0 License. See `LICENSE` for details.

