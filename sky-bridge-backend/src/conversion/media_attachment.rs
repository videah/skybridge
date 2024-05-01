use std::ops::Not;

use atrium_api::app::bsky::embed::images;
use mastodon_server_types::media_attachment::{
    MediaAttachment,
    MediaMetadata,
    MediaType,
    OriginalMetadata,
};

pub fn embed_to_media(view: &images::View) -> Vec<MediaAttachment> {
    view.images
        .iter()
        .map(|image| MediaAttachment {
            id: "1".to_string(),
            media_type: MediaType::Image,
            url: image.fullsize.clone(),
            preview_url: image.thumb.clone(),
            remote_url: None,
            meta: match &image.aspect_ratio {
                Some(meta) => MediaMetadata {
                    original: OriginalMetadata {
                        width: Some(meta.width),
                        height: Some(meta.height),
                        aspect: Some((meta.width / meta.height) as f32),
                    },
                },
                None => MediaMetadata::default(),
            },
            description: image.alt.is_empty().not().then(|| image.alt.clone()),
            blurhash: "".to_string(),
        })
        .collect()
}
