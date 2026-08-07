use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::{CreationChallengeResponse, PublicKeyCredential};
use zeroize::Zeroizing;

#[derive(Debug, Serialize, Deserialize)]
pub enum IpcRequest {
    GetStatus,
    RequestChallenge,
    VerifyAssertion { assertion: Box<PublicKeyCredential> },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IpcResponse {
    Status {
        has_secret: bool,
    },
    Challenge {
        options: Box<CreationChallengeResponse>,
    },
    Success {
        secret: Zeroizing<String>,
    },
    Error {
        message: String,
    },
}
