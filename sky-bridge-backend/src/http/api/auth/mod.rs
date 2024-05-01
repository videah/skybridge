use axum::{
    routing::{
        get,
        post,
    },
    Router,
};

use crate::http::ApiState;

pub mod authorize;
pub mod token;

pub fn router() -> Router<ApiState> {
    Router::new()
        .route("/oauth/token", post(token::get_token))
        .route("/oauth/authorize", get(authorize::sign_in_page))
        .route("/oauth/authorize", post(authorize::validate_auth))
}
