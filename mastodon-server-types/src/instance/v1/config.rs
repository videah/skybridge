use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct InstanceConfig {
    /// Limits related to accounts.
    pub accounts: AccountConfig,
    /// Limits related to authoring statuses.
    pub statuses: StatusConfig,
    /// Hints for which attachments will be accepted.
    pub media_attachments: MediaAttachmentConfig,
    /// Limits related to polls.
    pub polls: PollConfig,
}

/// Limits related to accounts.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct AccountConfig {
    /// The maximum number of featured tags allowed for each account.
    pub max_featured_tags: u32,
}

/// Limits related to authoring statuses.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct StatusConfig {
    /// The maximum number of allowed characters per status.
    pub max_characters: u32,
    /// The maximum number of media attachments that can be added to a status.
    pub max_media_attachments: u32,
    /// Each URL in a status will be assumed to be exactly this many characters.
    pub characters_reserved_per_url: u32,
}

/// Hints for which attachments will be accepted.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct MediaAttachmentConfig {
    /// Contains MIME types that can be uploaded.
    pub supported_mime_types: Vec<String>,
    /// The maximum size of any uploaded image, in bytes.
    pub image_size_limit: u32,
    /// The maximum number of pixels (width * height) for image uploads.
    pub image_matrix_limit: u32,
    /// The maximum size of any uploaded video, in bytes.
    pub video_size_list: u32,
    /// The maximum frame rate for any uploaded video.
    pub video_frame_rate_limit: u32,
    /// The maximum number of pixels (width * height) for video uploads.
    pub video_matrix_limit: u32,
}

/// Limits related to polls.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PollConfig {
    /// Each poll is allowed to have up to this many options.
    pub max_options: u32,
    /// Each poll option is allowed to have this many characters.
    pub max_characters_per_option: u32,
    /// The shortest allowed poll duration, in seconds.
    pub min_expiration: u32,
    /// The longest allowed poll duration, in seconds.
    pub max_expiration: u32,
}
