use std::{
    ops::Deref,
    sync::Arc,
};

use async_trait::async_trait;
use atrium_api::{
    agent::{
        AtpAgent,
        Session,
    },
    com::atproto::server::create_session,
};
use atrium_xrpc_client::reqwest::ReqwestClient;
use axum::{
    body::Body,
    extract::{
        FromRef,
        FromRequestParts,
    },
    http::{
        header::ToStrError,
        request::Parts,
        Response,
    },
    response::IntoResponse,
};
use log::debug;
use thiserror::Error;

use crate::{
    crypto::{
        HmacSigned,
        SignedDecodeError,
    },
    db::session::{
        DatabaseSessionStore,
        SessionRecord,
    },
    http::{
        api::auth::token::AccessToken,
        error::MastodonError,
        ApiState,
    },
};

#[derive(Debug, Error)]
pub enum SessionRetrievalError {
    #[error("a database error occurred: {0}")]
    DatabaseFailure(#[from] sqlx::Error),
    #[error("a valid token could not be decoded: {0}")]
    TokenDecodeFailure(#[from] SignedDecodeError),
    #[error("the request is missing an `Authorization` header")]
    MissingAuthHeader,
    #[error("the `Authorization` header contains invalid characters")]
    AuthHeaderInvalid(#[from] ToStrError),
    #[error("the `Authorization` header is missing the `Bearer ` prefix")]
    MissingBearerPrefix,
    #[error("failed to create a new session from bluesky API: {0}")]
    CreateSessionFailure(#[from] atrium_api::xrpc::error::Error<create_session::Error>),
}

impl IntoResponse for SessionRetrievalError {
    fn into_response(self) -> Response<Body> {
        match self {
            SessionRetrievalError::DatabaseFailure(e) => {
                MastodonError::unprocessable_entity("database_error", Some(e.to_string()))
                    .into_response()
            }
            SessionRetrievalError::TokenDecodeFailure(e) => {
                MastodonError::unauthorized("invalid_token", Some(e.to_string())).into_response()
            }
            SessionRetrievalError::MissingAuthHeader => {
                MastodonError::unauthorized("missing_auth_header", None).into_response()
            }
            SessionRetrievalError::AuthHeaderInvalid(e) => {
                MastodonError::unauthorized("invalid_auth_header", Some(e.to_string()))
                    .into_response()
            }
            SessionRetrievalError::MissingBearerPrefix => {
                MastodonError::unauthorized("missing_bearer_prefix", None).into_response()
            }
            SessionRetrievalError::CreateSessionFailure(e) => {
                MastodonError::unauthorized("create_session_failure", Some(e.to_string()))
                    .into_response()
            }
        }
    }
}

pub struct BlueskySession {
    agent: Arc<AtpAgent<DatabaseSessionStore, ReqwestClient>>,
    pub did: String,
}

impl Deref for BlueskySession {
    type Target = Arc<AtpAgent<DatabaseSessionStore, ReqwestClient>>;

    fn deref(&self) -> &Self::Target {
        &self.agent
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for BlueskySession
where
    ApiState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = SessionRetrievalError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let mut state = ApiState::from_ref(state);

        // Get bearer token from the request.
        let token = parts
            .headers
            .get("Authorization")
            .ok_or(SessionRetrievalError::MissingAuthHeader)?
            .to_str()?
            .to_string();

        // Strip the `Bearer ` prefix from the token.
        let token = token
            .strip_prefix("Bearer ")
            .ok_or(SessionRetrievalError::MissingBearerPrefix)?
            .to_string();
        debug!("Retrieved token: {:?}", token);

        // Try and decode the token into a `HmacSigned<AccessToken>`.
        let signed_token = HmacSigned::<AccessToken>::from_string(&token);
        debug!("Decoded token: {:?}", signed_token.is_ok());

        let signed_token = signed_token?;

        // First check if we already have an active session in memory, if we do just return it.
        if let Some(session) = state.sessions.get(&signed_token.data.did) {
            debug!("Retrieved session from memory...:");
            return Ok(BlueskySession {
                agent: session.clone(),
                did: signed_token.data.did,
            });
        }

        // If we don't, try adding a new session agent to the memory store.
        let agent = Arc::new(AtpAgent::new(
            ReqwestClient::new("https://bsky.social"),
            DatabaseSessionStore::new(signed_token.data.did.clone(), state.db.clone()),
        ));
        debug!("Created new agent, trying to login...");

        // First, try re-using the session, if this doesn't work we try creating a new one.
        let record =
            SessionRecord::get_from_did(&state.db.clone(), &signed_token.data.did.clone()).await?;

        let existing_session = match record {
            Some(record) => Some(Session {
                access_jwt: record.access_jwt,
                did: record.did,
                did_doc: None,
                email: None,
                email_confirmed: None,
                handle: record.handle,
                refresh_jwt: record.refresh_jwt,
            }),
            None => None,
        };

        if let Some(session) = existing_session {
            if agent.resume_session(session).await.is_ok() {
                // If we're successful, insert the new session into the memory store. The backing
                // session is stored in the database with `DatabaseSessionStore`.
                state
                    .sessions
                    .insert(signed_token.data.did.clone(), agent.clone());

                return Ok(BlueskySession {
                    agent,
                    did: signed_token.data.did,
                });
            }
        }

        agent
            .login(
                &signed_token.data.identifier,
                &signed_token.data.app_password,
            )
            .await?;
        debug!("Logged in successfully... adding to memory store.");

        // If we're successful, insert the new session into the memory store. The backing session is
        // stored in the database with `DatabaseSessionStore`.
        state
            .sessions
            .insert(signed_token.data.did.clone(), agent.clone());

        Ok(BlueskySession {
            agent,
            did: signed_token.data.did,
        })
    }
}
