use askama::Template;
use mastodon_server_types::oauth::authorize::AuthorizeRequestParams;

use crate::crypto::HmacSigned;

#[derive(Template)]
#[template(path = "sign_in.html")]
pub struct SignInPage {
    pub form_data: HmacSigned<AuthorizeRequestParams>,
}
