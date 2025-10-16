use crate::errors::Result;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct RefreshToken {
    pub sub: i32,
    pub jti: Uuid,
    iat: usize,
    pub exp: usize,
}

impl RefreshToken {
    pub fn new(sub: i32) -> Result<Self> {
        let exp_duration: i64 = env::var("JWT_REFRESH_TOKEN_EXPIRATION")?.parse()?;
        let current = OffsetDateTime::now_utc();
        tracing::debug!("current.unix_timestamp(): {}", current.unix_timestamp());

        Ok(Self {
            sub,
            jti: Uuid::now_v7(),
            iat: current.unix_timestamp() as usize,
            exp: (current + Duration::milliseconds(exp_duration)).unix_timestamp() as usize,
        })
    }

    pub fn generate(&self) -> Result<String> {
        let secret = env::var("JWT_SECRET")?;

        let token = jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;
        Ok(token)
    }

    pub fn hash(token: &str) -> String {
        hex::encode(Sha256::digest(token))
    }

    pub fn parse(token: &[u8]) -> Result<Self> {
        let secret = env::var("JWT_SECRET")?;

        let claim = jsonwebtoken::decode::<RefreshToken>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(claim.claims)
    }
}
