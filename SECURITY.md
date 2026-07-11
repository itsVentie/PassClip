## Security Model

`PassClip` operates under the following security assumptions:
1. **Zero Disk Persistence:** No unencrypted or encrypted secrets are written to persistent storage. All data resides in volatile memory (RAM).
2. **Memory Zeroization:** Cryptographic keys and sensitive plaintext segments are explicitly overwritten with zeroes when dropped to mitigate memory forensic exploitation.
3. **Local Scope:** The IPC channel and WebAuthn verification server bind strictly to the local loopback interface (`127.0.0.1`).

## Supported Versions

Only the latest release version receives security updates and patches.

## Reporting a Vulnerability

Do not open public GitHub issues for security vulnerabilities. 

Email security reports directly to: **[bugs@ventie.dev]**

Please include:
- A detailed description of the vulnerability.
- Proof of Concept (PoC) code or steps to reproduce.
- Potential impact analysis.

An acknowledgement will be sent within 48 hours, followed by a coordinated disclosure timeline.