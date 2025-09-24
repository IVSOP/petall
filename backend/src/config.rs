use std::time::Duration;

use anyhow::Context;
use clap::Parser;
use jsonwebtoken::{DecodingKey, EncodingKey};

#[derive(Parser)]
pub struct Config {
    #[arg(long, env)]
    pub postgres_host: String,
    #[arg(long, env, default_value_t = 5432)]
    pub postgres_port: u16,
    #[arg(long, env)]
    pub postgres_user: String,
    #[arg(long, env)]
    pub postgres_password: String,
    #[arg(long, env)]
    pub postgres_db: String,
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
