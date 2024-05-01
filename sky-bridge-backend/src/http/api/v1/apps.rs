use axum::{
    extract::State,
    Json,
};
use data_encoding::BASE64;
use mastodon_server_types::application::{
    Application,
    ApplicationInfo,
    CreateAppParams,
};
use ring::rand::SecureRandom;

use crate::http::{
    form_data::FormData,
    ApiState,
};

pub async fn create_app(
    State(state): State<ApiState>,
    FormData(app): FormData<CreateAppParams>,
) -> Json<Application> {
    // Base64-encode the client name to generate a client ID
    let client_id = BASE64.encode(&app.client_name.as_bytes());

    // Generate a random 32-byte secret key
    let mut secret_bytes = [0u8; 32];
    state
        .secure_rng
        .fill(&mut secret_bytes)
        .expect("RNG failure");
    let client_secret = hex::encode(secret_bytes);

    let application = Application {
        info: ApplicationInfo {
            name: app.client_name,
            website: app.website,
        },
        client_id: Some(client_id),
        client_secret: Some(client_secret),
        vapid_key: "".to_string(),
    };

    Json(application)
}
