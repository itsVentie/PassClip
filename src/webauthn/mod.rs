pub mod authenticator;

pub use authenticator::PasskeyAuthenticator;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;
use webauthn_rs::prelude::PasskeyAuthentication;

pub struct AuthSessionManager {
    sessions: Mutex<HashMap<Uuid, PasskeyAuthentication>>,
}

impl AuthSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    pub fn store_session(&self, session: PasskeyAuthentication) -> Uuid {
        let session_id = Uuid::new_v4();
        if let Ok(mut guard) = self.sessions.lock() {
            guard.insert(session_id, session);
        }
        session_id
    }

    pub fn retrieve_session(&self, session_id: &Uuid) -> Option<PasskeyAuthentication> {
        if let Ok(mut guard) = self.sessions.lock() {
            guard.remove(session_id)
        } else {
            None
        }
    }
}