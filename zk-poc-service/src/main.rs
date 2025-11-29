use std::{net::SocketAddr, sync::Arc};

use anyhow::Context;
use axum::{
    Json, Router, debug_handler,
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
};
use common::EnergyRecord;
use rand::distributions::{Alphanumeric, DistString};
use reqwest::StatusCode;
use serde::Serialize;
use tokio::net::TcpListener;
use tracing::info;
use url::Url;
use uuid::Uuid;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Address to bind the listener to
    #[arg(long, env, default_value = "0.0.0.0:3002")]
    listener: SocketAddr,

    /// URL to fetch energy records from
    #[arg(
        long,
        env,
        default_value = "http://localhost:8080/energy-record-unauthenticated/"
    )]
    url: Url,
}

#[derive(Clone)]
struct AppState {
    url: Arc<Url>,
}

impl AppState {
    async fn fetch_energy_record(&self, energy_record_id: Uuid) -> anyhow::Result<EnergyRecord> {
        reqwest::get(self.url.join(&energy_record_id.to_string())?)
            .await
            .context("Failed to fetch energy record")?
            .json::<EnergyRecord>()
            .await
            .context("Failed to parse energy record")
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let state = AppState {
        url: Arc::new(args.url),
    };

    let app = Router::new()
        .route("/validate/{uuid}", get(validate))
        .with_state(state);

    info!("Starting zk poc in {}", args.listener);

    let listener = TcpListener::bind(args.listener)
        .await
        .context("Failed to bind listener")?;

    tokio::select! {
        _ = axum::serve(listener, app) => {}
        _ = tokio::signal::ctrl_c() => {}
    }

    Ok(())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ValidateResponse {
    proof: String,
    energy_record: EnergyRecord,
}

#[debug_handler]
async fn validate(
    State(state): State<AppState>,
    Path(energy_record_id): Path<Uuid>,
) -> impl IntoResponse {
    let random_proof_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

    let Ok(energy_record) = state.fetch_energy_record(energy_record_id).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    Json(ValidateResponse {
        proof: random_proof_string,
        energy_record,
    })
    .into_response()
}
