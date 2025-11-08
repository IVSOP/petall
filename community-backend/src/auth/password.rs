use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use thiserror::Error;

pub fn hash_password(password: &str) -> Result<String, Argon2Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(Argon2Error)?
        .to_string())
}

pub fn verify_password(test_password: &str, hash: &str) -> Result<bool, Argon2Error> {
    let parsed_hash = PasswordHash::new(hash).map_err(Argon2Error)?;
    Ok(Argon2::default()
        .verify_password(test_password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[derive(Debug, Error)]
#[error("argon2: {0}")]
pub struct Argon2Error(argon2::password_hash::Error);
