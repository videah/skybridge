use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    account::{
        role::Role,
        Account,
        Field,
    },
    status::Visibility,
};

/// Contains extra information to be used with API methods that verify credentials and update
/// credentials.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CredentialAccount {
    /// The original [Account] data/information.
    #[serde(flatten)]
    pub account: Account,
    /// An extra attribute that contains source values to be used with API methods that verify
    /// credentials and update credentials.
    pub source: Source,
    /// The role assigned to the currently authorized user.
    pub role: Role,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Source {
    /// Profile bio, in plain-text instead of in HTML.
    pub note: String,
    /// Metadata about the account.
    pub fields: Vec<Field>,
    /// The default post privacy to be used for new statuses.
    pub privacy: Visibility,
    /// Whether new statuses should be marked sensitive by default.
    pub sensitive: bool,
    /// The default posting language for new statuses. Should be an ISO 639-1 language code or an
    /// empty string.
    pub language: String,
    /// The number of pending follow requests.
    pub follow_requests_count: u32,
}
