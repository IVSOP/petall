use crate::{AppState, auth};
use axum::{
    Router,
    routing::{delete, get, post},
};
use tower_http::trace::TraceLayer;

pub mod community;
pub mod user;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/communities", get(community::get_communities))
        .route("/community/{id}", get(community::get_community))
        .route(
            "/community/{community_id}/users",
            get(community::get_community_users),
        )
        .route("/community/register", post(community::register_community))
        .route(
            "/community/{community_id}/user",
            post(community::register_user_community),
        )
        .route(
            "/community/{community_id}/user/{user_id}",
            delete(community::remove_user_community),
        )
        .route(
            "/community/{community_id}/energy/{user_id}",
            get(user::get_user_energies),
        )
        .route("/users", get(user::get_users))
        .route(
            "/user/{user_id}",
            get(user::get_user),
        )
        .route(
            "/user/{user_id}/communities",
            get(user::get_user_communities),
        )
        .nest("/auth", auth::router::router().with_state(state.clone()))
        .with_state(state.clone())
        .layer(TraceLayer::new_for_http())
}
