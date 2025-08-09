use std::{sync::Arc, time::SystemTime};

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    api::results::Success,
    db::to_unix_timestamp,
    error::ModelError,
    ext_comm,
    session::{
        session_jwt::{SessionJWTData, SessionJWTService},
        session_store::{ConfirmSessionDb, InitSessionDb, SessionDb, SessionStore},
    },
};

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
}

/// implements direct conversion from SessionJWTData to SessionData
impl From<SessionJWTData> for SessionData {
    fn from(item: SessionJWTData) -> Self {
        Self {
            address: item.sub,
            sent_at: item.iat,
            expires_at: item.exp,
        }
    }
}

/// implements direct conversion from Session to SessionData
impl From<Session> for SessionData {
    fn from(item: Session) -> Self {
        Self {
            address: item.address,
            sent_at: item.sent_at,
            expires_at: item.expires_at,
        }
    }
    
}

/// implements direct conversion from 

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
    jwt_service: Arc<SessionJWTService>,
}

impl SessionService {
    pub fn new(store: SessionStore, ext_comm: ext_comm::ExtComm, jwt_service: Arc<SessionJWTService>) -> Self {
        Self { store, ext_comm, jwt_service }
    }

    /// Initializes a session by storing the address, code, and sent time in the database
    pub async fn signin_session(&self, item: InitSession) -> Result<Success, ModelError> {
        let sent_at: i64 = to_unix_timestamp(SystemTime::now()).try_into().unwrap_or(0);
        let expires_at = sent_at + (self.jwt_service.jwt_adapter.exp_secs as i64);
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
    pub async fn verify_session(&self, item: &ConfirmSession) -> Result<(SessionData, String), ModelError> {
        let db_session = self.store.confirm_session(item.into()).await?;
        let session: Session = db_session.into();

        self.jwt_service.generate_token(session)
    }
}