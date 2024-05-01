use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PreviewType {
    /// Link OEmbed.
    Link,
    /// Photo OEmbed.
    Photo,
    /// Video OEmbed.
    Video,
    /// iFrame OEmbed. Not currently accepted, so won’t show up in practice.
    Rich,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PreviewCard {
    /// Location of linked resource.
    pub url: String,
    /// Title of linked resource.
    pub title: String,
    /// Description of preview.
    pub description: String,
    /// The type of the preview card.
    #[serde(rename = "type")]
    pub preview_type: PreviewType,
    /// The author of the original resource.
    pub author_name: String,
    /// A link to the author of the original resource.
    pub author_url: String,
    /// The provider of the original resource.
    pub provider_name: String,
    /// A link to the provider of the original resource.
    pub provider_url: String,
    /// HTML to be used for generating the preview card.
    pub html: String,
    /// Width of preview, in pixels.
    pub width: u64,
    /// Height of preview, in pixels.
    pub height: u64,
    /// Preview thumbnail.
    pub image: Option<String>,
    /// Used for photo embeds, instead of custom `html`.
    pub embed_url: String,
    /// A hash computed by the BlurHash algorithm, for generating colorful preview thumbnails when
    /// media has not been downloaded yet.
    pub blurhash: Option<String>,
}
