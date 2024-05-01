use axum::{
    extract::State,
    Json,
};
use mastodon_server_types::instance::v1::{
    config::{
        AccountConfig,
        InstanceConfig,
        MediaAttachmentConfig,
        PollConfig,
        StatusConfig,
    },
    Instance,
    InstanceURLs,
    Rule,
    Stats,
};

use crate::http::ApiState;

/// Obtain general information about the server.
/// <https://docs.joinmastodon.org/methods/instance/#v1>
pub async fn instance_info(State(state): State<ApiState>) -> Json<Instance> {
    let instance = Instance {
        uri: state.config.base_url.to_owned(),
        title: "SkyBridge",
        short_description: "A Mastodon app bridge for Bluesky.",
        description: "A Mastodon app bridge for Bluesky.",
        email: "videah@selfish.systems",
        version: "4.1.2",
        urls: InstanceURLs {
            streaming_api: "".to_string(),
        },
        // TODO: Retrieve these values from the database.
        stats: Stats {
            user_count: 0,
            status_count: 0,
            domain_count: 0,
        },
        thumbnail: None,
        languages: vec![],
        registrations: false,
        approval_required: true,
        invites_enabled: false,
        configuration: InstanceConfig {
            accounts: AccountConfig {
                max_featured_tags: 0,
            },
            statuses: StatusConfig {
                max_characters: 300,
                max_media_attachments: 4,
                characters_reserved_per_url: 23,
            },
            media_attachments: MediaAttachmentConfig {
                supported_mime_types: vec!["image/jpeg".to_string(), "image/png".to_string()],
                image_size_limit: 976560,
                image_matrix_limit: 5000 * 5000,
                video_size_list: 0,
                video_frame_rate_limit: 0,
                video_matrix_limit: 0,
            },
            polls: PollConfig {
                max_options: 0,
                max_characters_per_option: 20,
                min_expiration: 1,
                max_expiration: 10000,
            },
        },
        contact_account: None,
        rules: vec![Rule {
            id: "1",
            text: "Report all issues to the SkyBridge repo!",
        }],
    };

    Json(instance)
}
