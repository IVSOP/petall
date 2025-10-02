use crate::{AppState, auth};
use axum::Router;
use tower_http::trace::TraceLayer;

pub mod community;
pub mod user;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(community::router().with_state(state.clone()))
        .merge(user::router().with_state(state.clone()))
        .nest("/auth", auth::router::router().with_state(state.clone()))
        .with_state(state.clone())
        .layer(TraceLayer::new_for_http())
}
