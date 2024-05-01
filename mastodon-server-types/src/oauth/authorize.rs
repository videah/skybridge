#[cfg(feature = "axum_typed_multipart")]
use axum_typed_multipart::TryFromMultipart;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "axum_typed_multipart", derive(TryFromMultipart))]
pub struct AuthorizeRequestParams {
    /// Should be set equal to `code`.
    pub response_type: String,
    /// The client ID, obtained during app registration.
    pub client_id: String,
    /// Set a URI to redirect the user to. If this parameter is set to urn:ietf:wg:oauth:2.0:oob
    /// then the authorization code will be shown instead. Must match one of the `redirect_uris`
    /// declared during app registration.
    pub redirect_uri: String,
    /// List of requested OAuth scopes, separated by spaces (or by pluses, if using query
    /// parameters). Must be a subset of `scopes` declared during app registration. If not
    /// provided, defaults to `read`.
    pub scope: Option<String>,
    /// Forces the user to re-login, which is necessary for authorizing with multiple accounts from
    /// the same instance.
    pub force_login: Option<bool>,
    /// The ISO 639-1 two-letter language code to use while rendering the authorization form.
    pub lang: Option<String>,
}
