use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Filter {
    /// The ID of the Filter in the database.
    pub id: String,
    /// A title given by the user to name the filter.
    pub title: String,
    /// The contexts in which the filter should be applied.
    pub context: Vec<FilterContext>,
    /// When the filter should no longer be applied.
    /// Set to `None` for a filter that never expires.
    pub expires_at: Option<DateTime<Utc>>,
    /// The action to be taken when a status matches this filter.
    pub filter_action: FilterAction,
    /// The keywords grouped under this filter.
    pub keywords: Vec<FilterKeyword>,
    /// The statuses grouped under this filter.
    pub statuses: Vec<FilterStatus>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FilterContext {
    /// Home timelines and lists.
    Home,
    /// Notifications timeline.
    Notifications,
    /// Public timelines.
    Public,
    /// Expanded thread of a detailed status.
    Thread,
    /// When viewing a profile.
    Account,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FilterAction {
    /// Show a warning that identifies the matching filter by `title`, and allow the user to expand
    /// the filtered status. This is the default (and unknown values should be treated as
    /// equivalent to `Warn`).
    Warn,
    /// Do not show this status if it is received.
    Hide,
}

impl Default for FilterAction {
    fn default() -> Self {
        FilterAction::Warn
    }
}

/// Represents a keyword that, if matched, should cause the filter action to be taken.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FilterKeyword {
    /// The ID of the FilterKeyword in the database.
    pub id: String,
    /// The phrase to be matched against.
    pub keyword: String,
    /// Should the filter consider word boundaries?
    /// [See implementation guidelines for filters](https://docs.joinmastodon.org/api/guidelines/#filters).
    pub whole_word: bool,
}

/// Represents a status ID that, if matched, should cause the filter action to be taken.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FilterStatus {
    /// The ID of the FilterStatus in the database.
    pub id: String,
    /// The ID of the Status that will be filtered.
    pub status_id: String,
}

/// Represents a filter whose keywords matched a given status.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FilterResult {
    /// The filter that was matched.
    pub filter: Filter,
    /// The keyword within the filter that was matched.
    pub keyword_matches: Vec<String>,
    /// The status ID within the filter that was matched.
    pub status_matches: Vec<String>,
}
