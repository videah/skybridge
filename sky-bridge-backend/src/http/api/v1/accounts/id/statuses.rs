use atrium_api::app::bsky::feed::get_author_feed;
use axum::{
    extract::{
        Path,
        State,
    },
    Json,
};
use mastodon_server_types::status::Status;

use crate::{
    conversion::post,
    db::accounts::AccountRecord,
    http::{
        error::MastodonError,
        sessions::extractor::BlueskySession,
        ApiState,
    },
    snowflake::SnowflakeID,
};

/// Statuses posted to the given account.
/// <https://docs.joinmastodon.org/methods/accounts/#statuses>
pub async fn account_posts(
    State(state): State<ApiState>,
    Path(id): Path<SnowflakeID>,
    session: BlueskySession,
) -> Result<Json<Vec<Status>>, MastodonError> {
    let record = AccountRecord::find_by_snowflake(&state.db, id).await?;
    match record {
        Some(account) => {
            let account = session
                .api
                .app
                .bsky
                .feed
                .get_author_feed(get_author_feed::Parameters {
                    actor: account.did.clone(),
                    cursor: None,
                    filter: None,
                    limit: None,
                })
                .await?;

            let mut posts = Vec::new();
            let mut tx = state.db.begin().await?;
            for post in account.feed {
                let post = post::feed_view_to_post(&mut tx, &state.snowflake, &post).await?;
                posts.push(post);
            }

            tx.commit().await?;

            Ok(Json(posts))
        }
        None => Err(MastodonError::not_found("Record not found", None)),
    }
}
