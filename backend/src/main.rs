use std::{net::IpAddr, str::FromStr, sync::Arc};

use sqlx::PgPool;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

mod seeds;
mod config;
mod router;
mod community;
mod error;
use crate::config::Config;

#[derive(Parser)]
#[command(name = "petall")]
struct Cli {
    #[command(flatten)]
    config: config::Config,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Seed {
        #[arg(long, default_value_t = 5)]
        communities: usize,
        #[arg(long, default_value_t = 5)]
        managers: usize,
        #[arg(long, default_value_t = 5)]
        users: usize,
        #[arg(long, default_value_t = 10)]
        energy_transfers: usize,
    }
}

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
    pub config: Arc<Config>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
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

    if let Some(Commands::Seed {communities, managers, users, energy_transfers }) = cli.command {
        let community_ids = seeds::seed_community(&pg_pool, communities).await?;
      
        let _manager_ids = seeds::seed_manager(&pg_pool, &community_ids, managers).await?;

        let _users_ids = seeds::seed_user(&pg_pool, &community_ids, users).await?;
        
        let _energy_transfer = seeds::seed_energytransfer(
            &pg_pool,
            &community_ids,
            &_users_ids,
            energy_transfers
        ).await?;
    };

    // save pgpool somewhere!!!!!

    let addr = (IpAddr::from_str(&config.bind_ip).unwrap(), config.bind_port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to port");

    let state = AppState {
        pg_pool,
        config: Arc::new(config),
    };

    tokio::select! {
        _ = axum::serve(listener, router::router(state)) => {}
        _ = tokio::signal::ctrl_c() => {}
    }

    Ok(())
}
