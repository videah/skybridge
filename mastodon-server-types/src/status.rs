use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    account::Account,
    application::ApplicationInfo,
    emoji::CustomEmoji,
    filter::FilterResult,
    media_attachment::MediaAttachment,
    poll::Poll,
    preview_card::PreviewCard,
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    /// Visible to everyone, shown in public timelines.
    Public,
    /// Visible to public, but not included in public timelines.
    Unlisted,
    /// Visible to followers only, and to any mentioned users.
    Private,
    /// Visible only to mentioned users.
    Direct,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Status {
    /// ID of the status in the database.
    pub id: String,
    /// URI of the status used for federation.
    pub uri: String,
    /// The date when this status was created.
    pub created_at: String,
    /// The account that authored this status.
    pub account: Account,
    /// HTML-encoded status content.
    pub content: String,
    /// Visibility of this status.
    pub visibility: Visibility,
    /// Is this status marked as sensitive content?
    pub sensitive: bool,
    /// Subject or summary line, below which status content is collapsed until expanded.
    pub spoiler_text: String,
    /// Media that is attached to this status.
    pub media_attachments: Vec<MediaAttachment>,
    /// The application used to post this status.
    pub application: Option<ApplicationInfo>,
    pub mentions: Vec<Mention>,
    pub tags: Vec<Tag>,
    pub emojis: Vec<CustomEmoji>,
    pub reblogs_count: i32,
    pub favourites_count: i32,
    pub replies_count: i32,
    pub url: Option<String>,
    pub in_reply_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub reblog: Option<Box<Status>>,
    pub poll: Option<Poll>,
    pub card: Option<PreviewCard>,
    pub language: Option<String>,
    pub text: Option<String>,
    pub edited_at: Option<DateTime<Utc>>,
    pub favourited: Option<bool>,
    pub reblogged: Option<bool>,
    pub muted: Option<bool>,
    pub bookmarked: Option<bool>,
    pub pinned: Option<bool>,
    pub filter: Option<Vec<FilterResult>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Mention {
    /// The account ID of the mentioned user.
    pub id: String,
    /// The username of the mentioned user.
    pub username: String,
    /// The location of the mentioned user's profile.
    pub url: String,
    /// The webfinger acct: URI of the mentioned user. Equivalent to `username` for local users, or
    /// `username@domain` for remote users.
    pub acct: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Tag {
    /// The value of the hashtag after the # sign.
    pub name: String,
    /// A link to the hashtag on the instance.
    pub url: String,
}
