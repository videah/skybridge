//! Types and conversions for errors returned by the HTTP server.

use std::fmt::Debug;

use axum::{
    body::Body,
    http::{
        Response,
        StatusCode,
    },
    response::IntoResponse,
    Json,
};
use log::error;
use serde::Serialize;

use crate::db::RecordError;

/// A Mastodon JSON error response in the format of the Mastodon API, with a message and optional
/// description alongside an appropriate status code.
#[derive(Debug, Serialize)]
pub struct MastodonError {
    /// The error message.
    pub error: &'static str,
    /// A longer description of the error, mainly provided with the OAuth API.
    pub error_description: Option<String>,
    /// The HTTP status code of the error. This is not included in the response body, it is instead
    /// handled by the [`IntoResponse`] trait.
    #[serde(skip_serializing)]
    status_code: StatusCode,
}

#[allow(dead_code)]
impl MastodonError {
    pub fn new(
        status_code: StatusCode,
        error: &'static str,
        error_description: Option<String>,
    ) -> Self {
        Self {
            error,
            error_description,
            status_code,
        }
    }

    /// Constructs a response for a `BAD REQUEST (400)` Mastodon error.
    ///
    /// Use this when the request made by the client cannot be processed due to client-side errors
    /// such as malformed request syntax, invalid request message framing, or deceptive request
    /// routing.
    pub fn bad_request(error: &'static str, error_description: Option<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, error, error_description)
    }

    /// Constructs a response for an `UNAUTHORIZED (401)` Mastodon error.
    ///
    /// Use this when the request lacks valid authentication credentials for the target resource or
    /// if the authentication credentials have not yet been provided.
    pub fn unauthorized(error: &'static str, error_description: Option<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, error, error_description)
    }

    /// Constructs a response for a `FORBIDDEN (403)` Mastodon error.
    ///
    /// Use this when the server understands the request but refuses to authorize it.
    /// This is typically used for failed permission checks.
    pub fn forbidden(error: &'static str, error_description: Option<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, error, error_description)
    }

    /// Constructs a response for a `NOT FOUND (404)` Mastodon error.
    ///
    /// Use this when the server cannot find the requested resource. This is typically used when the
    /// server has no matching route for the request URI.
    pub fn not_found(error: &'static str, error_description: Option<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, error, error_description)
    }

    /// Constructs a response for an `UNPROCESSABLE ENTITY (422)` Mastodon error.
    ///
    /// Use this when the server understands the content type of the request entity, and the syntax
    /// of the request entity is correct, but it was unable to process the contained instructions.
    /// This is often used for semantic errors in the request.
    pub fn unprocessable_entity(error: &'static str, error_description: Option<String>) -> Self {
        Self::new(StatusCode::UNPROCESSABLE_ENTITY, error, error_description)
    }

    /// Constructs a response for a `TOO MANY REQUESTS (429)` Mastodon error.
    ///
    /// Use this when the user has sent too many requests in a given amount of time ("rate
    /// limiting"). It is intended for use with rate-limiting schemes.
    pub fn too_many_requests(error: &'static str, error_description: Option<String>) -> Self {
        Self::new(StatusCode::TOO_MANY_REQUESTS, error, error_description)
    }

    /// Constructs a response for a `SERVICE UNAVAILABLE (503)` Mastodon error.
    ///
    /// Use this when the server is not ready to handle the request due to temporary overloading or
    /// maintenance. This implies that it is a temporary condition which will be alleviated after
    /// some delay.
    pub fn service_unavailable(error: &'static str, error_description: Option<String>) -> Self {
        Self::new(StatusCode::SERVICE_UNAVAILABLE, error, error_description)
    }

    /// Constructs a response for an `INTERNAL SERVER ERROR (500)` Mastodon error.
    ///
    /// Use this when the server encounters an unexpected condition that prevented it from
    /// fulfilling the request. This is a generic error message when no specific message is
    /// suitable.
    pub fn internal_server_error(error: &'static str, error_description: Option<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, error, error_description)
    }
}

impl IntoResponse for MastodonError {
    fn into_response(self) -> Response<Body> {
        error!("🐘❌ a Mastodon error occurred: {self:?}");
        (self.status_code.to_owned(), Json(self)).into_response()
    }
}

impl From<RecordError> for MastodonError {
    fn from(err: RecordError) -> Self {
        error!("📁❌ a record database upsert error occurred: {err}");
        match err {
            RecordError::DatabaseFailure(_) => {
                Self::internal_server_error("internal database error", None)
            }
            RecordError::SnowflakeFailure(_) => {
                Self::internal_server_error("snowflake error", None)
            }
            RecordError::UnexpectedRecordType => {
                Self::internal_server_error("unexpected record type", None)
            }
        }
    }
}

impl From<sqlx::Error> for MastodonError {
    fn from(err: sqlx::Error) -> Self {
        error!("💾❌ a database error occurred: {err}");
        match err {
            sqlx::Error::RowNotFound => Self::not_found("record not found", None),
            _ => Self::internal_server_error("internal database error", None),
        }
    }
}

impl<E: Debug> From<atrium_api::xrpc::error::Error<E>> for MastodonError {
    fn from(err: atrium_api::xrpc::error::Error<E>) -> Self {
        error!("🦋❌ an XRPC error occurred: {err}");
        match err {
            _ => Self::internal_server_error("unknown bluesky error", None),
        }
    }
}
