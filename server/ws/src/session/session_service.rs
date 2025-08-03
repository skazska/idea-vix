use std::{sync::Arc, time::{SystemTime}};

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{api::results::Success, db::to_unix_timestamp, error::ModelError, ext_comm, jwt_adapter::JwtAdapter, session::session_store::{ConfirmSessionDb, InitSessionDb, SessionDb, SessionStore}};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub id: i64,
    pub address: String,
    pub sent_at: u64,
    pub expires_at: u64,
}


#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct InitSession {
    #[validate(length(min = 1))]
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfirmSession {
    pub address: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionData {
    pub address: String,
    pub sent_at: u64,
    pub expires_at: u64,
    pub token: String,
}

/// implements direct conversion from ConfirmSession to ConfirmSessionDb
impl<'a> From<&'a ConfirmSession> for ConfirmSessionDb<'a> {
    fn from(item: &'a ConfirmSession) -> Self {
        Self {
            address: &item.address,
            code: &item.code,
        }
    }
}

/// implements direct conversion from SessionDb to Session
impl From<SessionDb> for Session {
    fn from(item: SessionDb) -> Self {
        let sent_at = item.sent_at.try_into().unwrap_or(0);
        let expires_at = item.expires_at.try_into().unwrap_or(0);

        Self {
            id: item.id,
            address: item.address,
            sent_at,
            expires_at,
        }
    }
}

/// Session data to be encoded in the JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionJWTData {
    pub aud: String,
    pub sub: String,
    pub exp: u64, // Expiration time in seconds since epoch
    pub iat: u64, // Issued at time in seconds since epoch
}

impl SessionJWTData {
    pub fn new(session: &Session) -> Self {
        let now = SystemTime::now();
        let iat = to_unix_timestamp(now) as u64;

        // Calculate expiration time (current time + expiration_secs)
        let exp = session.expires_at;

        Self {
            aud: "app".to_string(),
            sub: session.address.clone(),
            exp,
            iat,
        }
    }
}


pub struct SessionService {
    store: SessionStore,
    ext_comm: ext_comm::ExtComm,
    jwt_adapter: Arc<JwtAdapter>,
}

impl SessionService {
    pub fn new(store: SessionStore, ext_comm: ext_comm::ExtComm, jwt_adapter: Arc<JwtAdapter>) -> Self {
        Self { store, ext_comm, jwt_adapter }
    }

    /// Initializes a session by storing the address, code, and sent time in the database
    pub async fn signin_session(&self, item: InitSession) -> Result<Success, ModelError> {
        let sent_at: i64 = to_unix_timestamp(SystemTime::now()).try_into().unwrap_or(0);
        let expires_at = sent_at + (self.jwt_adapter.exp_secs as i64);
        let code: String = "some_code".to_string(); // This should be generated or provided

        self.ext_comm.send(&format!("Code to initialize session for {} is {}", item.address, code)).await?;

        let init_db = InitSessionDb {
            address: &item.address,
            code: &code,
            sent_at,
            expires_at,
        };
        
        self.store.init_session(init_db).await?;

        Ok(Success { success: true })
    }

    /// Confirms a session by checking the address and code
    pub async fn verify_session(&self, item: &ConfirmSession) -> Result<SessionData, ModelError> {
        let db_session = self.store.confirm_session(item.into()).await?;
        let session: Session = db_session.into();
        
        // Generate JWT token
        let token = self.jwt_adapter.generate_token(&SessionJWTData::new(&session))?;

        Ok(SessionData {
            address: session.address,
            sent_at: session.sent_at,
            expires_at: session.expires_at,
            token,
        })
    }
}