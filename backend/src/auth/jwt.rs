use crate::error::AppError;
use crate::error::AppResult;
use anyhow::Context;
use chrono::Utc;
use clap::Parser;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::time::Duration;
use uuid::Uuid;

#[derive(Parser)]
pub struct JwtConfig {
    #[arg(long, env, value_parser = parse_duration)]
    pub access_token_max_age: Duration,
    #[arg(long, env, value_parser = parse_duration)]
    pub refresh_token_max_age: Duration,
    #[arg(long, env, value_parser = load_decoding_key_from_file)]
    pub access_token_public_key: DecodingKey,
    #[arg(long, env, value_parser = load_encoding_key_from_file)]
    pub access_token_private_key: EncodingKey,
    #[arg(long, env, value_parser = load_decoding_key_from_file)]
    pub refresh_token_public_key: DecodingKey,
    #[arg(long, env, value_parser = load_encoding_key_from_file)]
    pub refresh_token_private_key: EncodingKey,
}

fn parse_duration(s: &str) -> anyhow::Result<Duration> {
    let number = s.parse::<u64>().context("Failed to parse number")?;
    Ok(Duration::from_secs(60 * number))
}

fn load_decoding_key_from_file(path: &str) -> anyhow::Result<DecodingKey> {
    let content = std::fs::read(path).context("Failed to read file")?;
    Ok(DecodingKey::from_rsa_pem(&content).context("Failed to parse RSA PEM")?)
}

fn load_encoding_key_from_file(path: &str) -> anyhow::Result<EncodingKey> {
    let content = std::fs::read(path).context("Failed to read file")?;
    Ok(EncodingKey::from_rsa_pem(&content).context("Failed to parse RSA PEM")?)
}

#[derive(Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
    pub token_id: Uuid,
    pub exp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: Uuid,
    pub token_id: Uuid,
    pub exp: i64,
}

pub fn create_access_token(
    jwt_config: &JwtConfig,
    token_id: Uuid,
    participant_id: Uuid,
) -> AppResult<String> {
    let age = jwt_config.access_token_max_age;
    let expiration = Utc::now().add(age).timestamp();

    let claims = AccessTokenClaims {
        sub: participant_id,
        token_id,
        exp: expiration,
    };

    let key = &jwt_config.access_token_private_key;
    let token = jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, key)?;
    Ok(token)
}

pub fn create_refresh_token(
    jwt_config: &JwtConfig,
    token_id: Uuid,
    participant_id: Uuid,
) -> AppResult<String> {
    let age = jwt_config.refresh_token_max_age;
    let expiration = Utc::now().add(age).timestamp();

    let claims = RefreshTokenClaims {
        sub: participant_id,
        token_id,
        exp: expiration,
    };

    let key = &jwt_config.refresh_token_private_key;
    let token = jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, key)?;
    Ok(token)
}

fn decode_token<T: serde::de::DeserializeOwned>(token: &str, key: &DecodingKey) -> AppResult<T> {
    let token = jsonwebtoken::decode::<T>(token, key, &Validation::new(Algorithm::RS256))?;
    Ok(token.claims)
}

pub fn decode_access_token(jwt_config: &JwtConfig, token: &str) -> AppResult<AccessTokenClaims> {
    let token: AccessTokenClaims = decode_token(token, &jwt_config.access_token_public_key)?;
    validate_expiration_date(token.exp)?;
    Ok(token)
}

pub fn decode_refresh_token(jwt_config: &JwtConfig, token: &str) -> AppResult<RefreshTokenClaims> {
    let token: RefreshTokenClaims = decode_token(token, &jwt_config.refresh_token_public_key)?;
    validate_expiration_date(token.exp)?;
    Ok(token)
}

fn validate_expiration_date(expiration: i64) -> AppResult<()> {
    let now = Utc::now().timestamp();
    if now > expiration {
        return Err(AppError::InvalidToken);
    }
    Ok(())
}
