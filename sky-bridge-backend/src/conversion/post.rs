use atrium_api::{
    app::bsky::feed::defs::{
        FeedViewPost,
        PostView,
        PostViewEmbedEnum::AppBskyEmbedImagesView,
    },
    records::Record,
};
use mastodon_server_types::{
    application::ApplicationInfo,
    status::{
        Status,
        Visibility,
    },
};
use sqlx::PgConnection;

use crate::{
    conversion::{
        account,
        media_attachment,
    },
    db::{
        posts::{
            PostInsert,
            PostRecord,
        },
        RecordError,
    },
    snowflake::SnowflakeGenerator,
};

/// Helper struct to store common viewer information about a post.
#[derive(Default)]
pub struct ViewerInfo {
    /// The current authenticated user has reposted this post.
    pub has_reposted: Option<bool>,
    /// The current authenticated user has liked this post.
    pub has_liked: Option<bool>,
    /// The current authenticated user cannot reply this post.
    pub is_restricted: Option<bool>,
}

pub async fn feed_view_to_post(
    db: &mut PgConnection,
    snowflake: &SnowflakeGenerator,
    view: &FeedViewPost,
) -> Result<Status, RecordError> {
    view_to_post(db, &snowflake, &view.post).await
}

pub async fn view_to_post(
    db: &mut PgConnection,
    snowflake: &SnowflakeGenerator,
    post: &PostView,
) -> Result<Status, RecordError> {
    let record = PostRecord::insert_or_update(
        db,
        snowflake,
        PostInsert {
            cid: post.cid.clone(),
            uri: post.uri.clone(),
            author_did: post.author.did.clone(),
        },
    )
    .await?;

    // Extract the text of the post.
    let text = match &post.record {
        Record::AppBskyFeedPost(post) => post.text.clone(),
        _ => {
            return Err(RecordError::UnexpectedRecordType);
        }
    };
    let escaped_text = html_escape::encode_text(&text);
    let content = format!("<p>{escaped_text}</p>");

    // Determine if the post is sensitive. For now, we just check if it has any labels at all,
    // but in the future we might want to check the labels for a specific value.
    let labels = post.labels.clone().unwrap_or_default();
    let is_sensitive = !labels.is_empty();

    // Construct a spoiler string from label values.
    let spoiler_text = labels
        .iter()
        .map(|label| label.val.clone())
        .collect::<Vec<String>>()
        .join(", ");

    let author = account::profile_basic_to_account(db, snowflake, &post.author).await?;

    // If there's a !no-unauthenticated label on the account, then the post is restricted.
    let account_labels = post.author.labels.clone().unwrap_or_default();
    let is_restricted = account_labels
        .iter()
        .any(|label| label.val == "!no-unauthenticated");

    let visibility = if is_restricted {
        Visibility::Unlisted
    } else {
        Visibility::Public
    };

    // Extract any images from the post and convert them to media attachments.
    let media_attachments = match &post.embed {
        Some(AppBskyEmbedImagesView(embed)) => media_attachment::embed_to_media(embed),
        _ => vec![],
    };

    // Get viewer information about the post, like if they've liked or reposted it.
    let viewer = match &post.viewer {
        Some(viewer) => ViewerInfo {
            has_reposted: Some(viewer.repost.is_some()),
            has_liked: Some(viewer.like.is_some()),
            is_restricted: viewer.reply_disabled,
        },
        None => ViewerInfo::default(),
    };

    // Construct a human-visitable URL for the post.
    let post_id = post.uri.split('/').last().unwrap_or_default();
    let url = format!(
        "https://bsky.app/profile/{}/post/{}",
        author.username, post_id
    );

    Ok(Status {
        id: record.id.0.to_string(),
        uri: url.clone(),
        created_at: post.indexed_at.clone(),
        account: author,
        content,
        visibility,
        sensitive: is_sensitive,
        spoiler_text,
        media_attachments,
        application: Some(ApplicationInfo {
            name: "Bluesky".to_string(),
            website: Some("https://bsky.social".to_string()),
        }),
        mentions: vec![],
        tags: vec![],
        emojis: vec![],
        reblogs_count: post.repost_count.unwrap_or(0),
        favourites_count: post.like_count.unwrap_or(0),
        replies_count: post.reply_count.unwrap_or(0),
        url: Some(url),
        in_reply_to_id: None,
        in_reply_to_account_id: None,
        reblog: None,
        poll: None,
        card: None,
        language: None,
        text: Some(text),
        edited_at: None,
        favourited: viewer.has_liked,
        reblogged: viewer.has_reposted,
        muted: None,
        bookmarked: None,
        pinned: None,
        filter: None,
    })
}
