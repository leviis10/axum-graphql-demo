use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;
use time::{Duration, OffsetDateTime};

#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    sub: i32,
    iat: usize,
    exp: usize,
}

impl AccessToken {
    pub fn new(sub: i32) -> crate::errors::Result<Self> {
        let exp_duration: i64 = env::var("JWT_ACCESS_TOKEN_EXPIRATION")?.parse()?;
        let current = OffsetDateTime::now_utc();
        tracing::debug!("current.unix_timestamp(): {}", current.unix_timestamp());

        Ok(Self {
            sub,
            iat: current.unix_timestamp() as usize,
            exp: (current + Duration::milliseconds(exp_duration)).unix_timestamp() as usize,
        })
    }

    pub fn generate(&self) -> crate::errors::Result<String> {
        let secret = env::var("JWT_SECRET")?;

        let token = jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;
        Ok(token)
    }
}
