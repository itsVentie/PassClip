use crate::vault::SlotMetadata;
use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::{PublicKeyCredential, RequestChallengeResponse};
use zeroize::Zeroizing;

#[derive(Debug, Serialize, Deserialize)]
pub enum IpcRequest {
    GetStatus,
    List,
    RequestChallenge { slot_id: Option<u32> },
    VerifyAssertion { assertion: Box<PublicKeyCredential> },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IpcResponse {
    Status {
        has_secret: bool,
        count: usize,
        max_slots: usize,
    },
    List {
        slots: Vec<SlotMetadata>,
    },
    Challenge {
        options: Box<RequestChallengeResponse>,
    },
    Success {
        secret: Zeroizing<String>,
    },
    Error {
        message: String,
    },
}