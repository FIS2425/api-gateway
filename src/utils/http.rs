use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::header::HeaderValue;
use hyper::http::response::Builder;
use hyper::http::response::Parts;
use hyper::Response;

pub type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

pub struct CorsResponse;

impl CorsResponse {
    pub fn builder() -> Builder {
        Response::builder()
            .header("Access-Control-Allow-Origin", "*")
            .header(
                "Access-Control-Allow-Methods",
                "GET, POST, OPTIONS, PUT, DELETE",
            )
            .header("Access-Control-Allow-Headers", "Content-Type")
    }

    pub fn from_parts(mut parts: Parts, body: Incoming) -> Response<BoxBody> {
        parts.headers.insert(
            "Access-Control-Allow-Origin",
            HeaderValue::from_str("*").unwrap(),
        );
        parts.headers.insert(
            "Access-Control-Allow-Methods",
            HeaderValue::from_str("GET, POST, OPTIONS, PUT, DELETE").unwrap(),
        );
        parts.headers.insert(
            "Access-Control-Allow-Headers",
            HeaderValue::from_str("Content-Type").unwrap(),
        );
        Response::from_parts(parts, body.boxed()).map(|body| body.boxed())
    }
}

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
