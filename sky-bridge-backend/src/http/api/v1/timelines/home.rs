use atrium_api::app::bsky::feed::get_timeline;
use axum::extract::State;
use axum_typed_multipart::TryFromMultipart;
use log::debug;
use mastodon_server_types::status::Status;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    conversion::post,
    http::{
        error::MastodonError,
        form_data::FormData,
        pagination::{
            PaginatedJson,
            PaginationHeader,
        },
        sessions::extractor::BlueskySession,
        ApiState,
    },
};

#[derive(TryFromMultipart, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TimelineParams {
    pub max_id: Option<String>,
    pub since_id: Option<String>,
    pub min_id: Option<String>,
    pub limit: Option<i32>,
    /// Non-standard parameter that Bluesky uses for pagination.
    pub next_cursor: Option<String>,
    pub prev_cursor: Option<String>,
}

impl Default for TimelineParams {
    fn default() -> Self {
        Self {
            max_id: None,
            since_id: None,
            min_id: None,
            limit: Some(20),
            next_cursor: None,
            prev_cursor: None,
        }
    }
}

/// View statuses from followed users and hashtags.
pub async fn home_timeline(
    State(state): State<ApiState>,
    session: BlueskySession,
    FormData(params): FormData<TimelineParams>,
) -> Result<PaginatedJson<Vec<Status>>, MastodonError> {
    let timeline = session
        .api
        .app
        .bsky
        .feed
        .get_timeline(get_timeline::Parameters {
            algorithm: None,
            cursor: params.next_cursor,
            limit: params.limit,
        })
        .await?;

    debug!("Timeline Cursor: {:?}", timeline.cursor);

    // TODO: improve concurrency
    // Currently running into an issue where performing the following in parallel
    // causes repeat snowflake generation. This seems like a bug in snowdon.
    let mut statuses = Vec::new();
    let mut tx = state.db.begin().await?;
    for view in &timeline.feed {
        if let Some(prev_cursor) = params.prev_cursor.clone() {
            // If we're paginating backwards, we need to skip the first post in the timeline.
            if view.post.indexed_at == prev_cursor {
                continue;
            }
        }

        let status = post::feed_view_to_post(&mut tx, &state.snowflake, view).await?;
        statuses.push(status);
    }

    // Get the indexed_at of the first post in the timeline.
    let prev_cursor = timeline
        .feed
        .first()
        .map(|view| view.post.indexed_at.clone());
    debug!("Prev Cursor: {:?}", prev_cursor);

    tx.commit().await?;

    if params.prev_cursor.is_some() {
        debug!("Previous Timeline Status Count: {:?}", statuses.len());
    }

    Ok(PaginatedJson {
        data: statuses,
        pagination: PaginationHeader {
            base_url: state.config.base_url.clone(),
            endpoint: "/api/v1/timelines/home".to_string(),
            next_cursor: timeline.cursor,
            prev_cursor: None,
        },
    })
}
