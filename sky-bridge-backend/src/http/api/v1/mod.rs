use axum::{
    routing::{
        get,
        post,
    },
    Router,
};

use crate::http::ApiState;

pub mod accounts;
pub mod apps;
pub mod instance;
pub mod timelines;

pub fn router() -> Router<ApiState> {
    Router::new()
        .nest("/api/v1/accounts", accounts::router())
        .nest("/api/v1/timelines", timelines::router())
        .route("/api/v1/apps", post(apps::create_app))
        .route("/api/v1/instance", get(instance::instance_info))
}
