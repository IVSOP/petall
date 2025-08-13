use std::{net::IpAddr, str::FromStr, sync::Arc};

use clap::Parser;
use sqlx::PgPool;
use anyhow::{Context, Result};


mod config;
mod router;
mod community;
mod error;
use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
    pub config: Arc<Config>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let config = config::Config::parse();

    let pg_options = sqlx::postgres::PgConnectOptions::new()
        .host(&config.pg_host)
        .port(config.pg_port)
        .username(&config.pg_user)
        .password(&config.pg_password)
        .database(&config.pg_database);

    let pg_pool = PgPool::connect_with(pg_options)
        .await
        .context("Failed to connect to Postgres")?;

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .context("Failed to run migrations")?;

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
