#[cfg(feature = "axum_typed_multipart")]
use axum_typed_multipart::TryFromMultipart;
use serde::{
    Deserialize,
    Serialize,
};

/// Represents an application that interfaces with the REST API to access accounts or post statuses.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Application {
    #[serde(flatten)]
    pub info: ApplicationInfo,
    /// Client ID key, to be used for obtaining OAuth tokens.
    pub client_id: Option<String>,
    /// Client secret key, to be used for obtaining OAuth tokens.
    pub client_secret: Option<String>,
    /// DEPRECATED: Used for Push Streaming API.
    pub vapid_key: String,
}

/// Public information about an application.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ApplicationInfo {
    /// The name of your application.
    pub name: String,
    /// The website associated with your application.
    pub website: Option<String>,
}

/// Form data for creating a new application.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "axum_typed_multipart", derive(TryFromMultipart))]
pub struct CreateAppParams {
    /// A name for your application.
    pub client_name: String,
    /// Where the user should be redirected after authorization. To display the authorization code
    /// to the user instead of redirecting to a web page, use urn:ietf:wg:oauth:2.0:oob in this
    /// parameter.
    pub redirect_uris: String,
    /// Space separated list of scopes. If none is provided, defaults to read. See [OAuth Scopes](https://docs.joinmastodon.org/api/oauth-scopes)
    /// for a list of possible scopes.
    #[cfg_attr(feature = "axum_typed_multipart", form_data(default))]
    pub scopes: Option<String>,
    /// A URL to the homepage of your app
    pub website: Option<String>,
}

impl Default for CreateAppParams {
    fn default() -> Self {
        Self {
            client_name: String::new(),
            redirect_uris: String::new(),
            scopes: Some("read".to_string()),
            website: None,
        }
    }
}
