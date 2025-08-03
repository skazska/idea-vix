use crate::error::ModelError;

/// Common communication utilities.

mod email;

pub struct ExtComm {
    email: email::Email,
}

impl ExtComm {
    pub fn new() -> Self {
        Self {
            email: email::Email::new(),
        }
    }

    pub async fn send(&self, message: &str) -> Result<(), ModelError> {
        self.email.send(message).await
    }
}