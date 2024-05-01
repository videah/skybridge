use axum::Router;

use crate::http::ApiState;

pub mod auth;
pub mod v1;

pub fn router() -> Router<ApiState> {
    Router::<ApiState>::new()
        .merge(auth::router())
        .merge(v1::router())
}
