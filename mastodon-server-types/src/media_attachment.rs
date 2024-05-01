use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    /// Unsupported or unrecognized file type.
    Unknown,
    /// Static image.
    Image,
    /// Looping, soundless animation.
    Gifv,
    /// Video clip.
    Video,
    /// Audio track.
    Audio,
}

/// Represents a file or media attachment that can be added to a status.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct MediaAttachment {
    /// The ID of the attachment in the database.
    pub id: String,
    /// The type of the attachment.
    #[serde(rename = "type")]
    pub media_type: MediaType,
    /// The location of the original full-size attachment.
    pub url: String,
    /// The location of a scaled-down preview of the attachment.
    pub preview_url: String,
    /// The location of the full-size original attachment on the remote website.
    pub remote_url: Option<String>,
    /// Metadata returned by Paperclip. Only includes image size data for now.
    pub meta: MediaMetadata,
    /// Alternate text that describes what is in the media attachment, to be used for the visually
    /// impaired or when media attachments do not load.
    pub description: Option<String>,
    /// A hash computed by the BlurHash algorithm, for generating colorful preview thumbnails when
    /// media has not been downloaded yet.
    pub blurhash: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct MediaMetadata {
    /// Metadata about the original resolution media attachment.
    pub original: OriginalMetadata,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct OriginalMetadata {
    /// The width of the media attachment in pixels.
    pub width: Option<i32>,
    /// The height of the media attachment in pixels.
    pub height: Option<i32>,
    /// The aspect ratio of the media attachment.
    pub aspect: Option<f32>,
}
