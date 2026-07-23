use crate::config::AppConfig;
use crate::crypto::SecureVault;
use crate::ipc::protocol::{IpcRequest, IpcResponse};
use arboard::Clipboard;
use notify_rust::Notification;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use webauthn_rs::{prelude::*, WebauthnBuilder};

#[cfg(windows)]
use tokio::net::windows::named_pipe::ServerOptions;
#[cfg(windows)]
use windows::core::HSTRING;
#[cfg(windows)]
use windows::Security::Credentials::UI::{UserConsentVerificationResult, UserConsentVerifier};

const PIPE_NAME: &str = r"\\.\pipe\passclip";
#[cfg(unix)]
const SOCKET_PATH: &str = "/tmp/passclip.sock";

fn get_webauthn() -> Webauthn {
    let cfg = AppConfig::load();
    let rp_id = &cfg.rp_id;
    let rp_origin = Url::parse(&cfg.rp_origin).unwrap();

    WebauthnBuilder::new(rp_id, &rp_origin)
        .unwrap()
        .rp_name(&cfg.rp_name)
        .build()
        .unwrap()
}

fn send_notification(summary: &str, body: &str) {
    let cfg = AppConfig::load();
    if cfg.enable_notifications {
        let _ = Notification::new()
            .summary(summary)
            .body(body)
            .appname("PassClip")
            .show();
    }
}

pub async fn run_server(vault: Arc<Mutex<SecureVault>>) {
    #[cfg(windows)]
    {
        let mut is_first = true;
        loop {
            let server_result = ServerOptions::new()
                .first_pipe_instance(is_first)
                .create(PIPE_NAME);

            let mut server = match server_result {
                Ok(s) => s,
                Err(_) => {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    is_first = false;
                    continue;
                }
            };

            if server.connect().await.is_ok() {
                is_first = false;
                let vault_clone = Arc::clone(&vault);
                tokio::spawn(async move {
                    let mut buf = vec![0u8; 4096];
                    if let Ok(n) = server.read(&mut buf).await {
                        if n > 0 {
                            let response = handle_request(&buf[..n], vault_clone).await;
                            let res_bytes = serde_json::to_vec(&response).unwrap();
                            let _ = server.write_all(&res_bytes).await;
                        }
                    }
                });
            }
        }
    }

    #[cfg(unix)]
    {
        let _ = std::fs::remove_file(SOCKET_PATH);
        let listener = tokio::net::UnixListener::bind(SOCKET_PATH).unwrap();
        loop {
            if let Ok((mut stream, _)) = listener.accept().await {
                let vault_clone = Arc::clone(&vault);
                tokio::spawn(async move {
                    let mut buf = vec![0u8; 4096];
                    if let Ok(n) = stream.read(&mut buf).await {
                        if n > 0 {
                            let response = handle_request(&buf[..n], vault_clone).await;
                            let res_bytes = serde_json::to_vec(&response).unwrap();
                            let _ = stream.write_all(&res_bytes).await;
                        }
                    }
                });
            }
        }
    }
}

async fn verify_user_presence() -> bool {
    #[cfg(windows)]
    {
        let prompt = HSTRING::from("PassClip: Подтвердите личность для доступа к секрету");
        
        tokio::task::spawn_blocking(move || {
            match UserConsentVerifier::RequestVerificationAsync(&prompt) {
                Ok(operation) => match operation.get() {
                    Ok(result) => result == UserConsentVerificationResult::Verified,
                    Err(_) => false,
                },
                Err(_) => false,
            }
        })
        .await
        .unwrap_or(false)
    }
    #[cfg(not(windows))]
    {
        true
    }
}

async fn handle_request(req_bytes: &[u8], vault: Arc<Mutex<SecureVault>>) -> IpcResponse {
    if let Ok(request) = serde_json::from_slice::<IpcRequest>(req_bytes) {
        let mut vault = vault.lock().await;
        let wa = get_webauthn();

        match request {
            IpcRequest::GetStatus => IpcResponse::Status {
                has_secret: vault.has_secret(),
            },
            IpcRequest::RequestChallenge => {
                if !vault.has_secret() {
                    return IpcResponse::Error {
                        message: "No secret isolated".to_string(),
                    };
                }
                match wa.start_passkey_authentication(&[]) {
                    Ok((options, auth_state)) => {
                        vault.current_auth = Some(auth_state);
                        IpcResponse::Challenge {
                            options: Box::new(options),
                        }
                    }
                    Err(e) => IpcResponse::Error {
                        message: format!("WebAuthn Err: {}", e),
                    },
                }
            }
            IpcRequest::VerifyAssertion { assertion: _ } => {
                if vault.current_auth.take().is_none() {
                    return IpcResponse::Error {
                        message: "No active auth session".to_string(),
                    };
                }

                if !verify_user_presence().await {
                    return IpcResponse::Error {
                        message: "User consent verification failed or was canceled".to_string(),
                    };
                }

                match vault.reveal() {
                    Ok(secret) => {
                        let secret_to_check = secret.clone();
                        let mut previous_content = String::new();

                        if let Ok(mut clipboard) = Clipboard::new() {
                            if let Ok(text) = clipboard.get_text() {
                                previous_content = text;
                            }
                        }

                        send_notification(
                            "Secret Restored",
                            "The secret is now in your clipboard.",
                        );

                        tokio::spawn(async move {
                            tokio::time::sleep(Duration::from_secs(30)).await;

                            if let Ok(mut clipboard) = Clipboard::new() {
                                if let Ok(current_text) = clipboard.get_text() {
                                    if current_text == secret_to_check {
                                        let _ = clipboard.set_text(previous_content);
                                        send_notification(
                                            "Clipboard Cleared",
                                            "The volatile secret has been wiped and previous content restored.",
                                        );
                                    }
                                }
                            }
                        });

                        IpcResponse::Success { secret }
                    }
                    Err(_) => IpcResponse::Error {
                        message: "Decryption failed".to_string(),
                    },
                }
            }
        }
    } else {
        IpcResponse::Error {
            message: "Invalid IPC request protocol".to_string(),
        }
    }
}

pub async fn send_client_request(request: IpcRequest) -> Result<IpcResponse, String> {
    let req_bytes = serde_json::to_vec(&request).unwrap();
    let mut buf = vec![0u8; 4096];

    #[cfg(windows)]
    {
        use tokio::net::windows::named_pipe::ClientOptions;
        let mut client = ClientOptions::new()
            .open(PIPE_NAME)
            .map_err(|e| format!("Failed to connect to daemon: {}", e))?;

        client.write_all(&req_bytes).await.map_err(|e| e.to_string())?;
        let n = client.read(&mut buf).await.map_err(|e| e.to_string())?;
        serde_json::from_slice(&buf[..n]).map_err(|e| e.to_string())
    }

    #[cfg(unix)]
    {
        let mut stream = tokio::net::UnixStream::connect(SOCKET_PATH)
            .await
            .map_err(|e| format!("Failed to connect to daemon: {}", e))?;

        stream.write_all(&req_bytes).await.map_err(|e| e.to_string())?;
        let n = stream.read(&mut buf).await.map_err(|e| e.to_string())?;
        serde_json::from_slice(&buf[..n]).map_err(|e| e.to_string())
    }
}