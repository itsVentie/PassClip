mod config;
mod crypto;
mod daemon;
mod ipc;

use clap::{Parser, Subcommand};
use crypto::SecureVault;
use daemon::{run_monitor, spawn_tray, SingleInstanceGuard};
use ipc::protocol::IpcRequest;
use ipc::server::{run_server, send_client_request};
use log::{error, info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use webauthn_rs::prelude::*;

#[derive(Parser)]
#[command(name = "passclip")]
#[command(about = "Secures clipboard data", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Daemon,
    Pop,
    Status,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Daemon => {
            info!("Initializing PassClip daemon...");

            let _single_instance = match SingleInstanceGuard::acquire() {
                Ok(guard) => guard,
                Err(err) => {
                    error!("{}", err);
                    std::process::exit(1);
                }
            };

            let vault = Arc::new(Mutex::new(SecureVault::new()));

            let monitor_vault = Arc::clone(&vault);
            std::thread::spawn(move || {
                run_monitor(monitor_vault);
            });

            info!("IPC server starting up...");
            let server_vault = Arc::clone(&vault);
            tokio::spawn(async move {
                run_server(server_vault).await;
            });

            info!("Launching system tray...");
            if let Err(e) = spawn_tray() {
                error!("System tray encountered an error: {}", e);
            }
        }

        Commands::Status => {
            info!("Querying daemon status...");
            match send_client_request(IpcRequest::GetStatus).await {
                Ok(ipc::protocol::IpcResponse::Status { has_secret }) => {
                    info!("Vault contains secret: {}", has_secret);
                }
                Ok(ipc::protocol::IpcResponse::Error { message }) => {
                    error!("Daemon returned error: {}", message);
                }
                Err(e) => {
                    error!("Failed to communicate with daemon: {}", e);
                }
                _ => warn!("Received unexpected response from daemon."),
            }
        }
        Commands::Pop => {
            info!("Initiating challenge request...");
            match send_client_request(IpcRequest::RequestChallenge).await {
                Ok(ipc::protocol::IpcResponse::Challenge { options }) => {
                    info!("Passkey challenge received. Authenticating...");

                    let raw_bytes = options.public_key.challenge.clone();
                    let json_payload = serde_json::json!({
                        "id": serde_json::to_value(&raw_bytes).unwrap_or_default(),
                        "rawId": raw_bytes,
                        "response": {
                            "authenticatorData": raw_bytes,
                            "clientDataJSON": raw_bytes,
                            "signature": [],
                            "userHandle": null
                        },
                        "extensions": {},
                        "type": "public-key"
                    });

                    match serde_json::from_value::<PublicKeyCredential>(json_payload) {
                        Ok(assertion) => {
                            match send_client_request(IpcRequest::VerifyAssertion {
                                assertion: Box::new(assertion),
                            })
                            .await
                            {
                                Ok(ipc::protocol::IpcResponse::Success { secret }) => {
                                    match arboard::Clipboard::new() {
                                        Ok(mut clipboard) => {
                                            if clipboard.set_text(secret.as_str()).is_ok() {
                                                info!("Passkey verified! Secret restored to clipboard.");
                                            } else {
                                                error!("Failed to write secret to clipboard.");
                                            }
                                        }
                                        Err(e) => {
                                            error!("Failed to access system clipboard: {}", e)
                                        }
                                    }
                                }
                                Ok(ipc::protocol::IpcResponse::Error { message }) => {
                                    error!("Authentication failed: {}", message);
                                }
                                Err(e) => error!("Failed to send assertion verification: {}", e),
                                _ => warn!("Unexpected response during assertion verification."),
                            }
                        }
                        Err(e) => error!("Failed to construct assertion payload: {}", e),
                    }
                }
                Ok(ipc::protocol::IpcResponse::Error { message }) => {
                    error!("Daemon error: {}", message);
                }
                Err(e) => error!("Failed to reach daemon: {}", e),
                _ => warn!("Unexpected IPC challenge response."),
            }
        }
    }
}
