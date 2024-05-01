use axum::{
    response::{
        IntoResponse,
        Response,
    },
    Json,
};
use headers::{
    Header,
    HeaderMapExt,
};
use http::HeaderName;
use serde::Serialize;

static PAGINATION_HEADER: &'static str = "link";
static HEADER_NAME: HeaderName = HeaderName::from_static(PAGINATION_HEADER);

/// The `Link` header used by apps for pagination.
///
/// Currently, Bluesky is limited to only paginating forward and cannot paginate backwards. This is
/// because rather than paginating by snowflake ID, Bluesky paginates by a timestamp cursor.
///
/// For now, the `prev` link will simply point to the base endpoint retrieving the latest results,
/// and the `next` link will point to the endpoint with a non-standard cursor parameter. It's
/// possible some clients won't like this.
///
/// TODO: Implement a more standard pagination method when possible.
/// Relevant discussion: <https://github.com/bluesky-social/atproto/discussions/1834>
/// Relevant issue: <https://github.com/bluesky-social/social-app/issues/976>
pub struct PaginationHeader {
    /// The base URL of the SkyBridge instance.
    pub base_url: String,
    /// The endpoint which this header is being returned from.
    pub endpoint: String,
    /// The cursor timestamp returned by the appropriate Bluesky API endpoint.
    /// This is non-standard and some clients might yell at us for it.
    pub next_cursor: Option<String>,
    /// A locally constructed cursor for the previous page, which is the indexed_at timestamp of
    /// the first post in the current page. Used to try and strip repeat posts on calls to `prev`.
    pub prev_cursor: Option<String>,
}

impl Header for PaginationHeader {
    fn name() -> &'static HeaderName {
        &HEADER_NAME
    }

    fn decode<'i, I>(_values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        // We don't have to ever decode this header, so we'll just return a dummy value.
        Ok(PaginationHeader {
            base_url: "".to_string(),
            endpoint: "".to_string(),
            next_cursor: None,
            prev_cursor: None,
        })
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<http::HeaderValue>,
    {
        let endpoint = format!("{}{}", self.base_url, self.endpoint);
        let next = match &self.next_cursor {
            Some(next_cursor) => format!(r#"<{endpoint}?next_cursor={next_cursor}>; rel="next""#),
            None => "".to_string(),
        };

        let prev = match &self.prev_cursor {
            Some(prev_cursor) => format!(r#"<{endpoint}?prev_cursor={prev_cursor}>; rel="prev""#),
            None => format!(r#"<{endpoint}>; rel="prev""#),
        };

        let value = format!("{next}, {prev}");
        let header_value = http::HeaderValue::from_str(&value).unwrap();
        values.extend(std::iter::once(header_value));
    }
}

/// Helper struct to wrap a JSON axum response with a pagination header.
pub struct PaginatedJson<T: Serialize> {
    pub data: T,
    pub pagination: PaginationHeader,
}

impl<T: Serialize> IntoResponse for PaginatedJson<T> {
    fn into_response(self) -> Response {
        let mut response = Json(self.data).into_response();
        response.headers_mut().typed_insert(self.pagination);
        response
    }
}
