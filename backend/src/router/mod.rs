use crate::AppState;
use tower_http::trace::TraceLayer;
use axum::{
    Router, routing::{get, post, delete},
};

pub mod community;
pub mod participant;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route(
            "/communities",
            get(community::get_communities))
        .route(
            "/community/{id}",
            get(community::get_community))
        .route(
            "/community/register",
            post(community::register_community))
        .route(
            "/community/{community_id}/participant",
            post(community::register_participant_community),
        )
        .route(
            "/community/{community_id}/participant/{participant_id}",
            delete(community::remove_participant_community),
        )
        .route(
            "/community/{community_id}/energy/{participant_id}",
            get(participant::get_participant_energies),
        )
        .route(
            "/participants",
            get(participant::get_participants))
        .route(
            "/participant/{participant_id}",
            get(participant::get_participant))
        .route(
            "/participant/{participant_id}/communities",
            get(participant::get_participant_communities),
        )
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
