#[cfg(feature = "axum_typed_multipart")]
use axum_typed_multipart::{
    TryFromField,
    TryFromMultipart,
};
use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

/// What kind of OAuth2 flow to use when obtaining a token.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "axum_typed_multipart", derive(TryFromField))]
#[cfg_attr(
    feature = "axum_typed_multipart",
    try_from_field(rename_all = "snake_case")
)]
pub enum GrantType {
    /// For end-users
    AuthorizationCode,
    /// For bots and other single-user applications
    PasswordGrant,
    /// For applications that do not act on behalf of users
    ClientCredentials,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "axum_typed_multipart", derive(TryFromMultipart))]
pub struct TokenRequestParams {
    /// Set equal to `authorization_code` if code is provided in order to gain user-level access.
    /// Otherwise, set equal to `client_credentials` to obtain app-level access only.
    pub grant_type: GrantType,
    /// A user authorization code, obtained via [GET /oauth/authorize](https://docs.joinmastodon.org/methods/oauth/#authorize).
    pub code: Option<String>,
    /// The client ID, obtained during app registration.
    pub client_id: String,
    /// The client secret, obtained during app registration.
    pub client_secret: String,
    /// Set a URI to redirect the user to. If this parameter is set to urn:ietf:wg:oauth:2.0:oob
    /// then the token will be shown instead. Must match one of the `redirect_uris` declared during
    /// app registration.
    pub redirect_uri: String,
    /// List of requested OAuth scopes, separated by spaces (or by pluses, if using query
    /// parameters). If `code` was provided, then this must be equal to the `scope` requested from
    /// the user. Otherwise, it must be a subset of `scopes` declared during app registration. If
    /// not provided, defaults to `read`.
    pub scope: Option<String>,
}

impl Default for TokenRequestParams {
    fn default() -> Self {
        Self {
            grant_type: GrantType::AuthorizationCode,
            code: None,
            client_id: String::new(),
            client_secret: String::new(),
            redirect_uri: String::new(),
            scope: Some("read".to_string()),
        }
    }
}

/// Represents an OAuth token used for authenticating with the API and performing actions.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Token {
    /// An OAuth token to be used for authorization.
    pub access_token: String,
    /// The OAuth token type. Mastodon uses `Bearer` tokens.
    pub token_type: String,
    /// The OAuth scopes granted by this token, space-separated.
    pub scope: String,
    /// When the token was generated.
    pub created_at: DateTime<Utc>,
}
