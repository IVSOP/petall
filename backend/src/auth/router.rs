use axum::{Router, debug_handler, routing::post};

pub fn router() -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_handler))
        .route("/logout", post(logout_handler))
        .route("/changepassword", post(change_password_handler))
}

#[debug_handler]
async fn register_handler() {}

#[debug_handler]
async fn login_handler() {}

#[debug_handler]
async fn refresh_handler() {}

#[debug_handler]
async fn logout_handler() {}

#[debug_handler]
async fn change_password_handler() {}
