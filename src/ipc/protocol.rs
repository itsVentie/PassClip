use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum IpcRequest {
    PopSecret,
    GetStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IpcResponse {
    Success { secret: String },
    Status { has_secret: bool },
    Error { message: String },
}