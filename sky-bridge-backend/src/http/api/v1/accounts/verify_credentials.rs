use atrium_api::app::bsky::actor::get_profile;
use axum::{
    extract::State,
    Json,
};
use mastodon_server_types::account::credentials::CredentialAccount;
use tracing::debug;

use crate::{
    conversion::account,
    http::{
        error::MastodonError,
        sessions::extractor::BlueskySession,
        ApiState,
    },
};

/// Test to make sure that the user token works.
/// <https://docs.joinmastodon.org/methods/accounts/#verify_credentials>
pub async fn verify(
    State(state): State<ApiState>,
    session: BlueskySession,
) -> Result<Json<CredentialAccount>, MastodonError> {
    debug!("Verifying credentials for user: {:?}", session.did);

    let profile = session
        .api
        .app
        .bsky
        .actor
        .get_profile(get_profile::Parameters {
            actor: session.did.clone(),
        })
        .await?;

    let account =
        account::profile_detailed_to_cred_account(&state.db, &state.snowflake, &profile).await?;
    Ok(Json(account))
}
