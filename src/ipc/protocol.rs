use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::RequestChallengeResponse;

#[derive(Debug, Serialize, Deserialize)]
pub enum IpcRequest {
    GetStatus,
    RequestChallenge,
    VerifyAssertion {
        assertion: Box<RequestChallengeResponse>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IpcResponse {
    Status { has_secret: bool },
    Challenge { options: Box<RequestChallengeResponse> },
    Success { secret: String },
    Error { message: String },
}