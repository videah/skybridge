use axum::{
    async_trait,
    extract::{
        FromRequest,
        FromRequestParts,
        Query,
        Request,
    },
    http::{
        header::CONTENT_TYPE,
        StatusCode,
    },
    response::IntoResponse,
    Form,
    Json,
    RequestExt,
};
use axum_typed_multipart::TypedMultipart;
use log::debug;
use serde_aux::prelude::serde_introspect;

use crate::http::error::MastodonError;

pub struct FormData<T>(pub T);

/// Produces a malformed request error when the request can be parsed but the content is not
/// what was expected.
///
/// TODO: Handle missing parameters on successful parsing.
fn malformed_error(_: impl IntoResponse) -> MastodonError {
    MastodonError::new(
        StatusCode::BAD_REQUEST,
        "invalid_request",
        Some("The request is missing a required parameter, includes an unsupported parameter value, or is otherwise malformed.".to_string()),
    )
}

#[async_trait]
impl<S, T> FromRequest<S> for FormData<T>
    where
        S: Send + Sync,
        Json<T>: FromRequest<()>,
        TypedMultipart<T>: FromRequest<()>,
        Form<T>: FromRequest<()>,
        Query<T>: FromRequestParts<()>,
        T: 'static,
        T: serde::de::DeserializeOwned,
{
    type Rejection = MastodonError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if let Some(content_type) = content_type {
            debug!("Content-Type: {}", content_type);
            if content_type.starts_with("application/json") {
                let Json(payload) = req.extract().await.map_err(malformed_error)?;
                return Ok(Self(payload));
            }

            if content_type.starts_with("multipart/form-data") {
                let TypedMultipart(payload) = req.extract().await.map_err(malformed_error)?;
                return Ok(Self(payload));
            }

            if content_type.starts_with("application/x-www-form-urlencoded") {
                let Form(payload) = req.extract().await.map_err(malformed_error)?;
                return Ok(Self(payload));
            }
        }

        if let Ok(Query(payload)) = req.extract().await.map_err(malformed_error)? {
            return Ok(Self(payload));
        }

        // Get the first field name of T using serde and use it as the error message.
        let fields = serde_introspect::<T>();
        let description = format!("Missing required parameter: {}", fields[0]);

        Err(MastodonError::bad_request(
            "invalid_request",
            Some(description),
        ))
    }
}
