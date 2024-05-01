use serde::{
    Deserialize,
    Serialize,
};

/// Represents a custom emoji.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CustomEmoji {
    /// The name of the custom emoji.
    pub shortcode: String,
    /// A link to the custom emoji.
    pub url: String,
    /// A link to a static copy of the custom emoji.
    pub static_url: String,
    /// Whether this Emoji should be visible in the picker or unlisted.
    pub visible_in_picker: bool,
    /// Used for sorting custom emoji in the picker.
    pub category: String,
}
