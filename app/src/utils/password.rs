use crate::errors::Result;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub fn argon_instance<'a>() -> Argon2<'a> {
    Argon2::default()
}

pub fn hash_password(password: &[u8]) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = argon_instance().hash_password(password, &salt)?.to_string();
    Ok(hashed_password)
}

pub fn compare_password(raw: &str, hashed: &str) -> Result<()> {
    let parsed_hash = PasswordHash::new(hashed)?;

    argon_instance().verify_password(raw.as_bytes(), &parsed_hash)?;

    Ok(())
}
