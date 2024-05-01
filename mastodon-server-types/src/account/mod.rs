pub mod credentials;
pub mod role;

use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::emoji::CustomEmoji;

/// Represents a user of Mastodon and their associated profile.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Account {
    /// The account id.
    /// Cast from an integer, but not guaranteed to be a number.
    pub id: String,
    /// The username of the account, not including domain.
    pub username: String,
    /// The Webfinger account URI. Equal to `username` for local users, or `username@domain` for
    /// remote users.
    pub acct: String,
    /// The location of the user’s profile page.
    pub url: String,
    /// The profile’s display name.
    pub display_name: String,
    /// The profile’s bio or description.
    pub note: String,
    /// An image icon that is shown next to statuses and in the profile.
    pub avatar: String,
    /// A static version of the avatar. Equal to `avatar` if its value is a static image; different
    /// if `avatar` is an animated GIF.
    pub avatar_static: String,
    /// An image banner that is shown above the profile and in profile cards.
    pub header: String,
    /// A static version of the header. Equal to `header` if its value is a static image; different
    /// if `header` is an animated GIF.
    pub header_static: String,
    /// Whether the account manually approves follow requests.
    pub locked: bool,
    /// Additional metadata attached to a profile as name-value pairs.
    pub fields: Vec<Field>,
    /// Custom emoji entities to be used when rendering the profile.
    pub emojis: Vec<CustomEmoji>,
    /// Indicates that the account may perform automated actions, may not be monitored, or
    /// identifies as a robot.
    pub bot: bool,
    /// Indicates that the account represents a `Group` actor.
    pub group: bool,
    /// Whether the account has opted into discovery features such as the profile directory.
    pub discoverable: Option<bool>,
    /// Whether the local user has opted out of being indexed by search engines.
    pub noindex: Option<bool>,
    /// Indicates that the profile is currently inactive and that its user has moved to a new
    /// account. Set to `None` if the account is suspended.
    pub moved: Box<Option<Self>>,
    /// An extra attribute returned only when an account is suspended.
    pub suspended: Option<bool>,
    /// An extra attribute returned only when an account is silenced. If true, indicates that the
    /// account should be hidden behind a warning screen.
    pub limited: Option<bool>,
    /// When the account was created.
    pub created_at: DateTime<Utc>,
    /// When the most recent status was posted. Set to `None` if no statuses have been posted.
    pub last_status_at: Option<DateTime<Utc>>,
    /// How many statuses are attached to this account.
    pub statuses_count: u32,
    /// The reported followers of this profile.
    pub followers_count: u32,
    /// The reported follows of this profile.
    pub following_count: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Field {
    /// The key of a given field’s key-value pair.
    pub name: String,
    /// The value associated with the `name` key.
    pub value: String,
    /// Timestamp of when the server verified a URL value for a rel=“me” link. Value is `None` if
    /// the server could not verify the URL.
    pub verified_at: Option<DateTime<Utc>>,
}
