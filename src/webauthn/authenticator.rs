use std::sync::Arc;
use url::Url;
use webauthn_rs::prelude::*;

pub struct PasskeyAuthenticator {
    wa: Arc<Webauthn>,
}

impl PasskeyAuthenticator {
    pub fn new(rp_id: &str, rp_origin: &str) -> Result<Self, WebauthnError> {
        let origin = Url::parse(rp_origin).map_err(|_| WebauthnError::Configuration)?;
        let builder = WebauthnBuilder::new(rp_id, origin)?;
        let wa = builder.build()?;
        Ok(Self { wa: Arc::new(wa) })
    }

    pub fn start_auth(
        &self,
        allow_credentials: Vec<Credential>,
    ) -> Result<(DiscoverableAuthenticationInstantiation, PasskeyAuthentication), WebauthnError> {
        self.wa.start_passkey_authentication(&allow_credentials)
    }

    pub fn verify_auth(
        &self,
        assertion: &PublicKeyCredential<AuthenticatorAssertionResponse>,
        auth_state: PasskeyAuthentication,
    ) -> Result<AuthenticationResult, WebauthnError> {
        self.wa.finish_passkey_authentication(assertion, auth_state)
    }
}