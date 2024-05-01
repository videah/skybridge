//! Configured values and limits for this website.

pub mod config;

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    account::Account,
    instance::v1::config::InstanceConfig,
};

/// Represents the software instance of Mastodon running on this domain.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Instance {
    /// The domain name of the instance.
    pub uri: String,
    /// The title of the website.
    pub title: &'static str,
    /// A short, plain-text description defined by the admin.
    pub short_description: &'static str,
    /// An HTML-permitted description of the Mastodon site.
    pub description: &'static str,
    /// An email that may be contacted for any inquiries.
    pub email: &'static str,
    /// The version of Mastodon installed on the instance.
    pub version: &'static str,
    /// URLs of interest for clients apps.
    pub urls: InstanceURLs,
    /// Statistics about how much information the instance contains.
    pub stats: Stats,
    /// Banner image for the website.
    pub thumbnail: Option<String>,
    /// Primary languages of the website and its staff.
    pub languages: Vec<&'static str>,
    /// Whether registrations are enabled.
    pub registrations: bool,
    /// Whether registrations require moderator approval.
    pub approval_required: bool,
    /// Whether invites are enabled.
    pub invites_enabled: bool,
    /// Configured values and limits for this website.
    pub configuration: InstanceConfig,
    /// A user that can be contacted, as an alternative to `email`.
    /// TODO: remove Option
    pub contact_account: Option<Account>,
    /// An itemized list of rules for this website.
    pub rules: Vec<Rule>,
}

/// Statistics about how much information the instance contains.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Stats {
    /// Total users on this instance.
    pub user_count: u32,
    /// Total statuses on this instance.
    pub status_count: u32,
    /// Total domains discovered by this instance.
    pub domain_count: u32,
}

/// URLs of interest for clients apps.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct InstanceURLs {
    /// The Websockets URL for connecting to the streaming API.
    pub streaming_api: String,
}

/// Represents a rule that server users should follow.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Rule {
    /// An identifier for the rule.
    pub id: &'static str,
    /// The rule to be followed.
    pub text: &'static str,
}
