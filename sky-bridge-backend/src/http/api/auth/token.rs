use atrium_api::com::atproto::identity::resolve_handle;
use axum::{
    extract::State,
    Json,
};
use chrono::Utc;
use mastodon_server_types::oauth::token::{
    GrantType,
    Token,
    TokenRequestParams,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    crypto,
    crypto::HmacSigned,
    http::{
        api::auth::authorize::AuthCode,
        error::MastodonError,
        form_data::FormData,
        ApiState,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    /// The identifier entered by the user, either an email or handle.
    pub identifier: String,
    /// The decentralized identifier (DID) of the user.
    pub did: String,
    /// The app password entered by the user, used to authenticate with bluesky.
    pub app_password: String,
}

pub async fn get_token(
    State(state): State<ApiState>,
    FormData(params): FormData<TokenRequestParams>,
) -> Result<Json<Token>, MastodonError> {
    let token = match params.grant_type {
        GrantType::AuthorizationCode => {
            // If the code is not provided, return an error.
            let Some(code) = params.code else {
                return Err(MastodonError::bad_request("invalid_request", None));
            };

            // Decode the signed code from the request.
            let signed_code = HmacSigned::<AuthCode>::from_string(&code).unwrap();

            // Resolve the handle to the users decentralized identifier (DID).
            // This gets passed later to the DatabaseSessionStore when creating a new session in the
            // BlueskySession extractor.
            let user = state
                .bsky
                .service
                .com
                .atproto
                .identity
                .resolve_handle(resolve_handle::Parameters {
                    handle: signed_code.data.identifier.clone(),
                })
                .await
                .map_err(|_| MastodonError::unauthorized("invalid_grant", None))?;

            // Construct the access token and sign it with the HMAC key.
            let access_token = AccessToken {
                identifier: signed_code.data.identifier,
                did: user.did,
                app_password: signed_code.data.app_password,
            };
            let signed_access_token = crypto::sign_and_pack(access_token, &state.hmac_key);

            Token {
                access_token: signed_access_token.to_string(),
                token_type: "Bearer".to_string(),
                scope: signed_code.data.scope,
                created_at: Utc::now(),
            }
        }
        GrantType::PasswordGrant => unimplemented!(),
        GrantType::ClientCredentials => Token {
            // We don't care about this access_token for now.
            // TODO: Handle this properly.
            access_token: "ZA-Yj3aBD8U8Cm7lKUp-lm9O9BmDgdhHzDeqsY8tlL0".to_string(),
            token_type: "Bearer".to_string(),
            scope: params.scope.unwrap_or("read".to_string()),
            created_at: Utc::now(),
        },
    };

    Ok(Json(token))
}
