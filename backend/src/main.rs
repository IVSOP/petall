use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use sqlx::PgPool;
use std::net::IpAddr;
use tracing::info;
use crate::seed::SeedSettings;

mod controller;
mod models;
mod router;
mod error;
mod seed;

#[derive(Parser)]
pub struct DatabaseConfig {
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
}

#[derive(Parser)]
#[command(name = "petall")]
struct Cli {
    #[command(flatten)]
    config: DatabaseConfig,
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
    Seed(SeedSettings),
}

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
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

    match cli.command {
        Command::Run { bind_ip, bind_port } => {
            let listener = tokio::net::TcpListener::bind((bind_ip, bind_port))
                .await
                .context("Failed to bind to port")?;

            let state = AppState { pg_pool };

            info!("Starting server on {}", listener.local_addr().unwrap());

            tokio::select! {
                _ = axum::serve(listener, router::router(state)) => {}
                _ = tokio::signal::ctrl_c() => {}
            }
        }
        Command::Seed(seed_settings) => {
            seed::run_seed(&pg_pool, seed_settings).await?;

            info!("Finished seeding");
        }
    }

    Ok(())
}
