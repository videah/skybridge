pub mod statuses;

use atrium_api::app::bsky::actor::get_profile;
use axum::{
    extract::{
        Path,
        State,
    },
    routing::get,
    Json,
    Router,
};
use mastodon_server_types::account::Account;

use crate::{
    conversion::account,
    db::accounts::AccountRecord,
    http::{
        error::MastodonError,
        sessions::extractor::BlueskySession,
        ApiState,
    },
    snowflake::SnowflakeID,
};

pub fn router() -> Router<ApiState> {
    Router::new()
        .route("/:id", get(get_account))
        .route("/:id/statuses", get(statuses::account_posts))
}

/// View information about a profile.
/// <https://docs.joinmastodon.org/methods/accounts/#get>
pub async fn get_account(
    State(state): State<ApiState>,
    Path(id): Path<SnowflakeID>,
    session: BlueskySession,
) -> Result<Json<Account>, MastodonError> {
    let record = AccountRecord::find_by_snowflake(&state.db, id).await?;
    log::debug!("Account Record: {:?}", record);
    match record {
        Some(account) => {
            let profile = session
                .api
                .app
                .bsky
                .actor
                .get_profile(get_profile::Parameters { actor: account.did })
                .await?;

            let account =
                account::profile_detailed_to_account(&state.db, &state.snowflake, &profile).await?;
            Ok(Json(account))
        }
        None => Err(MastodonError::not_found("Record not found", None)),
    }
}
