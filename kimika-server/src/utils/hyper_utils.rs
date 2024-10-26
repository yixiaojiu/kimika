use super::types;

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::Response;

pub fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub fn rejection_response<T: Into<Bytes>>(body: T) -> Response<types::BodyType> {
    Response::builder()
        .status(400)
        .header("Content-Type", "text/plain")
        .body(full(body))
        .unwrap()
}
