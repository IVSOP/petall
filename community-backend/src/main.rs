use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;
use std::net::IpAddr;
use tracing::info;

mod auth;
mod controller;
mod error;
mod models;
mod router;
mod seed;

#[derive(Parser)]
#[command(name = "petall")]
struct Cli {
    #[command(flatten)]
    config: Config,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Run {
        #[arg(long, env, default_value_t = IpAddr::from([0, 0, 0, 0]))]
        bind_ip: IpAddr,
        #[arg(long, env, default_value_t = 8080)]
        bind_port: u16,
    },
}

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
    #[arg(long, env)]
    pub google_client_id: String,
    #[arg(long, env)]
    pub google_client_secret: String,
    #[arg(long, env)]
    pub google_redirect_url: String,
    #[arg(long, env, value_parser = load_decoding_key_from_file)]
    pub validation_public_key: DecodingKey,
    #[arg(long, env, value_parser = load_encoding_key_from_file)]
    pub validation_private_key: EncodingKey,
}

fn load_decoding_key_from_file(path: &str) -> Result<DecodingKey, std::io::Error> {
    Ok(DecodingKey::from_rsa_pem(&std::fs::read(path)?).unwrap())
}

fn load_encoding_key_from_file(path: &str) -> Result<EncodingKey, std::io::Error> {
    Ok(EncodingKey::from_rsa_pem(&std::fs::read(path)?).unwrap())
}

#[derive(Clone)]
pub struct AppState {
    pg_pool: PgPool,
    google_oauth: auth::oauth::GoogleOAuthClient,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    let config = cli.config;

    let pg_options = sqlx::postgres::PgConnectOptions::new()
        .host(&config.postgres_host)
        .port(config.postgres_port)
        .username(&config.postgres_user)
        .password(&config.postgres_password)
        .database(&config.postgres_db);

    let pg_pool = PgPool::connect_with(pg_options)
        .await
        .context("Failed to connect to Postgres")?;

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .context("Failed to run migrations")?;

    let google_oauth = auth::oauth::GoogleOAuthClient::new(
        config.google_client_id,
        config.google_client_secret,
        config.google_redirect_url,
    )
    .context("Failed to initialize Google OAuth client")?;

    match cli.command {
        Command::Run { bind_ip, bind_port } => {
            let listener = tokio::net::TcpListener::bind((bind_ip, bind_port))
                .await
                .context("Failed to bind to port")?;

            let state = AppState {
                pg_pool,
                google_oauth,
            };

            let seeder = tokio::spawn(seed::run_periodic_seed_task(state.clone()));

            info!("Starting server on {}", listener.local_addr().unwrap());

            tokio::select! {
                _ = axum::serve(listener, router::router(state)) => {}
                _ = seeder => {},
                _ = tokio::signal::ctrl_c() => {}
            }
        }
    }

    Ok(())
}
