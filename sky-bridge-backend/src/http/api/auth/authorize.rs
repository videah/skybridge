use axum::{
    extract::State,
    http::{
        HeaderMap,
        StatusCode,
    },
    response::IntoResponse,
    Form,
};
use log::debug;
use mastodon_server_types::oauth::authorize::AuthorizeRequestParams;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    crypto,
    crypto::HmacSigned,
    html::SignInPage,
    http::{
        error::MastodonError,
        form_data::FormData,
        ApiState,
    },
};

pub async fn sign_in_page(
    State(state): State<ApiState>,
    FormData(params): FormData<AuthorizeRequestParams>,
) -> SignInPage {
    SignInPage {
        form_data: crypto::sign_and_pack(params, &state.hmac_key),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthForm {
    /// A signed object containing the parameters of the authorization request.
    pub params: HmacSigned<AuthorizeRequestParams>,
    pub bridge_password: String,
    pub identifier: String,
    pub app_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCode {
    pub client_id: String,
    pub scope: String,
    pub identifier: String,
    pub app_password: String,
}

/// Get sign-in information entered into the form by the user and process
/// it to redirect with an auth token code.
#[axum::debug_handler]
pub async fn validate_auth(
    State(state): State<ApiState>,
    Form(form): Form<AuthForm>,
) -> Result<impl IntoResponse, MastodonError> {
    debug!("Received form data: {:?}", form);
    let params = form.params.data;

    let code = AuthCode {
        client_id: params.client_id,
        scope: params.scope.unwrap_or_default(),
        identifier: form.identifier,
        app_password: form.app_password,
    };

    let signed_code = crypto::sign_and_pack(code, &state.hmac_key);
    let uri = format!("{}?code={signed_code}", params.redirect_uri);

    let mut headers = HeaderMap::new();
    headers.insert("Location", uri.parse().unwrap());
    let response = (StatusCode::FOUND, headers);

    Ok(response)
}
