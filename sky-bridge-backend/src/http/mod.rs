pub mod api;
pub mod error;
pub mod form_data;
pub mod pagination;
pub mod sessions;

use std::{
    collections::HashMap,
    sync::Arc,
};

use anyhow::Context;
use atrium_api::{
    agent::AtpAgent,
    client::AtpServiceClient,
};
use atrium_xrpc_client::reqwest::ReqwestClient;
use axum::Router;
use clap::Parser;
use log::info;
use ring::{
    hmac,
    rand::SystemRandom,
};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

use crate::{
    config::Config,
    db::session::DatabaseSessionStore,
    snowflake::SnowflakeGenerator,
};

/// The application state shared across all requests.
#[derive(Clone)]
pub struct ApiState {
    /// The application configuration processed from environment variables and command line.
    pub config: Arc<Config>,
    /// The connection pool for the PostgreSQL database.
    pub db: PgPool,
    /// A cryptographically secure random number generator.
    pub secure_rng: SystemRandom,
    /// The HMAC key used for signing and verifying access tokens.
    pub hmac_key: hmac::Key,
    /// A client for making unauthenticated requests to the Bluesky API.
    pub bsky: Arc<AtpServiceClient<ReqwestClient>>,
    /// A map of active sessions, indexed by their access tokens. Used to make authenticated
    /// requests to the Bluesky API on behalf of users.
    pub sessions: HashMap<String, Arc<AtpAgent<DatabaseSessionStore, ReqwestClient>>>,
    /// A generator for generating snowflake IDs guaranteed to be unique across the application.
    pub snowflake: Arc<SnowflakeGenerator>,
}

/// Sets up and starts the HTTP server and then listens for incoming requests.
pub async fn serve(db: PgPool) -> anyhow::Result<()> {
    let config = Config::parse();
    let hmac_key = hmac::Key::new(hmac::HMAC_SHA256, config.secret_key.as_ref());
    let generator = Arc::new(SnowflakeGenerator::default());

    let state = ApiState {
        config: Arc::new(config),
        db,
        secure_rng: SystemRandom::new(),
        hmac_key,
        bsky: Arc::new(AtpServiceClient::new(ReqwestClient::new(
            "https://bsky.social",
        ))),
        sessions: HashMap::new(),
        snowflake: generator,
    };

    let app = Router::<ApiState>::new()
        .merge(api::router())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    info!("☁️ Listening on {}", listener.local_addr()?);

    axum::serve(listener, app)
        .await
        .context("error running HTTP server")
}
