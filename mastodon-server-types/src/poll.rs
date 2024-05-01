use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::emoji::CustomEmoji;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Poll {
    /// The ID of the poll in the database.
    pub id: String,
    /// When the poll ends. If `None`, the poll does not end.
    pub expires_at: Option<DateTime<Utc>>,
    /// Is the poll currently expired?
    pub expired: bool,
    /// Does the poll allow multiple-choice answers?
    pub multiple: bool,
    /// How many votes have been received.
    pub votes_count: u64,
    /// How many unique accounts have voted on a multiple-choice poll.
    /// Should be `None` if `multiple` is `false`.
    pub voters_count: Option<u64>,
    /// Possible answers for the poll.
    pub options: Vec<PollOption>,
    /// Custom emoji to be used for rendering poll options.
    pub emojis: Vec<CustomEmoji>,
    /// When called with a user token, has the authorized user voted?
    pub voted: Option<bool>,
    /// When called with a user token, which options has the authorized user chosen? Contains an
    /// array of index values for `options`.
    pub own_votes: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PollOption {
    /// The text value of the poll option.
    pub title: String,
    /// The total number of received votes for this option.
    /// Should be `None` if results are not published yet.
    pub votes_count: Option<u64>,
}
