use clap::Parser;
use sqlx::PgPool;
use anyhow::{Context, Result};

mod config;

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

    // tokio::select! {
    //     _ = axum::serve(listener, router::router(state)) => {}
    //     _ = tokio::signal::ctrl_c() => {}
    // }

    Ok(())
}
