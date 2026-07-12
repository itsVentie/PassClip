mod crypto;
mod daemon;
mod ipc;

use crypto::SecureVault;
use clap::{Parser, Subcommand};
use ipc::protocol::IpcRequest;
use ipc::server::{run_server, send_client_request};
use std::sync::Arc;
use tokio::sync::Mutex;

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
    let cli = Cli::parse();

    match &cli.command {
        Commands::Daemon => {
            let vault = Arc::new(Mutex::new(SecureVault::new()));
            println!("PassClip daemon started.");

            let monitor_vault = Arc::clone(&vault);
            std::thread::spawn(move || {
                daemon::run_monitor(monitor_vault);
            });

            run_server(vault).await;
        }
        Commands::Status => {
            match send_client_request(IpcRequest::GetStatus).await {
                Ok(ipc::protocol::IpcResponse::Status { has_secret }) => {
                    println!("Vault contains secret: {}", has_secret);
                }
                _ => println!("Error: Unable to get status from daemon."),
            }
        }
        Commands::Pop => {
            match send_client_request(IpcRequest::PopSecret).await {
                Ok(ipc::protocol::IpcResponse::Success { secret }) => {
                    let mut clipboard = arboard::Clipboard::new().unwrap();
                    if clipboard.set_text(secret).is_ok() {
                        println!("[+] Secret successfully restored to clipboard.");
                    }
                }
                Ok(ipc::protocol::IpcResponse::Error { message }) => {
                    println!("Daemon error: {}", message);
                }
                _ => println!("Error: Failed to communicate with daemon."),
            }
        }
    }
}