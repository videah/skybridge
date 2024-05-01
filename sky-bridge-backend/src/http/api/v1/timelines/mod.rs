use axum::{
    routing::get,
    Router,
};

use crate::http::{
    api::v1::timelines::home::home_timeline,
    ApiState,
};

pub mod home;

pub fn router() -> Router<ApiState> {
    Router::new().route("/home", get(home_timeline))
}
