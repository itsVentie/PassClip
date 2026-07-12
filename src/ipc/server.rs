use crate::crypto::SecureVault;
use crate::ipc::protocol::{IpcRequest, IpcResponse};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(windows)]
use tokio::net::windows::named_pipe::ServerOptions;

const PIPE_NAME: &str = r"\\.\pipe\passclip";
#[cfg(unix)]
const SOCKET_PATH: &str = "/tmp/passclip.sock";

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
                    let mut buf = vec![0u8; 1024];
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
                    let mut buf = vec![0u8; 1024];
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

async fn handle_request(req_bytes: &[u8], vault: Arc<Mutex<SecureVault>>) -> IpcResponse {
    if let Ok(request) = serde_json::from_slice::<IpcRequest>(req_bytes) {
        let mut vault = vault.lock().await;
        match request {
            IpcRequest::GetStatus => IpcResponse::Status {
                has_secret: vault.has_secret(),
            },
            IpcRequest::PopSecret => match vault.reveal() {
                Ok(secret) => IpcResponse::Success { secret },
                Err(_) => IpcResponse::Error {
                    message: "No secret isolated or storage empty".to_string(),
                },
            },
        }
    } else {
        IpcResponse::Error {
            message: "Invalid IPC request protocol".to_string(),
        }
    }
}

pub async fn send_client_request(request: IpcRequest) -> Result<IpcResponse, String> {
    let req_bytes = serde_json::to_vec(&request).unwrap();
    let mut buf = vec![0u8; 1024];

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