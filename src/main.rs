mod config;
mod crypto;
mod daemon;
mod ipc;

use clap::{Parser, Subcommand};
use crypto::SecureVault;
use ipc::protocol::IpcRequest;
use ipc::server::{run_server, send_client_request};
use log::{error, info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use zeroize::Zeroizing;

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

            let vault = Arc::new(Mutex::new(SecureVault::new()));

            let monitor_vault = Arc::clone(&vault);
            std::thread::spawn(move || {
                daemon::run_monitor(monitor_vault);
            });

            info!("IPC server starting up...");
            run_server(vault).await;
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

                    match send_client_request(IpcRequest::VerifyAssertion { assertion: options }).await {
                        Ok(ipc::protocol::IpcResponse::Success { secret }) => {
                            let secure_secret = Zeroizing::new(secret);

                            match arboard::Clipboard::new() {
                                Ok(mut clipboard) => {
                                    if clipboard.set_text((*secure_secret).clone()).is_ok() {
                                        info!("Passkey verified! Secret restored to clipboard.");
                                    } else {
                                        error!("Failed to write secret to clipboard.");
                                    }
                                }
                                Err(e) => error!("Failed to access system clipboard: {}", e),
                            }
                        }
                        Ok(ipc::protocol::IpcResponse::Error { message }) => {
                            error!("Authentication failed: {}", message);
                        }
                        Err(e) => error!("Failed to send assertion verification: {}", e),
                        _ => warn!("Unexpected response during assertion verification."),
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