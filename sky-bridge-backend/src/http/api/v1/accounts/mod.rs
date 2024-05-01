use axum::{
    routing::get,
    Router,
};

use crate::http::ApiState;

pub mod id;
pub mod verify_credentials;

pub fn router() -> Router<ApiState> {
    Router::new()
        .nest("/", id::router())
        .route("/verify_credentials", get(verify_credentials::verify))
}
