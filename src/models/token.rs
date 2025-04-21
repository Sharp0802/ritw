use crate::models::User;
use chrono::prelude::*;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use crate::services::{Sign, SignError};

static TOKEN_DURATION: Duration = Duration::hours(24);

#[derive(Deserialize, Serialize)]
pub struct Token {
    id: String,
    due: DateTime<Utc>
}

impl Token {
    pub fn new(user: &User) -> Self {
        Self {
            id: user.id().to_string(),
            due: Utc::now().add(TOKEN_DURATION)
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn expired(&self) -> bool {
        self.due >= Utc::now()
    }
}

impl TryInto<Vec<u8>> for Token {
    type Error = SignError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let bytes = serde_json::to_string(&self).unwrap();
        Sign::encrypt(&bytes.as_bytes())
    }
}

impl TryFrom<&[u8]> for Token {
    type Error = SignError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let bytes = Sign::decrypt(value)?;
        serde_json::from_slice::<Token>(&bytes).map_err(|_| SignError)
    }
}
