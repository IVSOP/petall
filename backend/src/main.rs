use crate::seed::SeedSettings;
use anyhow::{Context, Result};
use chrono::{Timelike, Utc};
use clap::{Parser, Subcommand};
use sqlx::PgPool;
use tokio::time::{self, Instant};
use std::{net::IpAddr, time::Duration};
use tracing::{info, error};

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
    Seed(SeedSettings),
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
}

#[derive(Clone)]
pub struct AppState {
    pg_pool: PgPool,
    google_oauth: auth::oauth::GoogleOAuthClient,
}

/// Starts a scheduler that runs every quarter-hour (00, 15, 30, 45)
pub async fn periodic_seed(state: AppState) {
    // --- Compute the first tick aligned to next quarter-hour ---
    let now = Utc::now();
    let minutes = now.minute();
    let seconds = now.second();

    // Compute how many minutes until the next multiple of 15
    // This means this is only accurate to the minute. However, when records are inserted, seconds are always zeroed out
    let next_quarter = ((minutes / 15) + 1) * 15 % 60;
    let minutes_until_next = if next_quarter > minutes {
        next_quarter - minutes
    } else {
        60 - minutes
    };

    let initial_delay_secs = (minutes_until_next * 60 - seconds as u32) as u64;
    let start_instant = Instant::now() + Duration::from_secs(initial_delay_secs);

    // Create a 15-minute interval aligned to that start time
    let mut interval = time::interval_at(start_instant, Duration::from_secs(15 * 60));

    info!(
        "Scheduler will start in {:?} (at next quarter-hour boundary)",
        Duration::from_secs(initial_delay_secs)
    );

    loop {
        interval.tick().await;
        info!("Seeding records");

        if let Err(e) = state.insert_random_energy_records().await {
            error!("Error inserting random energy records: {}", e);
        }
    }
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

            let seeder = tokio::spawn(periodic_seed(state.clone()));

            info!("Starting server on {}", listener.local_addr().unwrap());

            tokio::select! {
                _ = axum::serve(listener, router::router(state)) => {}
                _ = seeder => {},
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
