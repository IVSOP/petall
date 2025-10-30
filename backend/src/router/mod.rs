use crate::AppState;
use crate::auth;
use axum::Router;
use axum::routing::put;
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

pub mod admin;
pub mod community;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route(
            "/admin/community",
            get(admin::get_admin_manageable_communities).post(admin::create_community),
        )
        .route(
            "/admin/community/{id}",
            get(admin::get_admin_community_information).patch(admin::edit_community_information),
        )
        .route(
            "/admin/community/{id}/manager",
            put(admin::add_manager_to_community).delete(admin::remove_manager_from_community),
        )
        .route(
            "/admin/community/{id}/user",
            put(admin::add_user_to_community).delete(admin::remove_user_from_community),
        )
        .route(
            "/community",
            get(community::get_communities_with_user_energy_records),
        )
        .route("/community/{id}", get(community::get_community_by_id))
        .route(
            "/community/{id}/energy",
            post(community::list_user_energy_records),
        )
        .route("/community/{id}/stats", post(community::get_stats))
        .nest("/auth", auth::router::router().with_state(state.clone()))
        .with_state(state.clone())
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
pub(crate) mod test_utils {
    use axum_test::TestServer;
    use sqlx::PgPool;

    use crate::{AppState, router::router};

    pub(crate) fn test_server(pg_pool: PgPool) -> TestServer {
        let google_oauth = crate::auth::oauth::GoogleOAuthClient::new(
            "test-client-id".to_string(),
            "test-client-secret".to_string(),
            "http://localhost:8080/auth/callback".to_string(),
        )
        .unwrap();

        let state = AppState {
            pg_pool,
            google_oauth,
        };
        let router = router(state);

        TestServer::new(router).unwrap()
    }
}
