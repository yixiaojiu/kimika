pub mod receiver;
pub mod sender;
pub mod udp;

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{Request, Response};

pub type RequestType = Request<hyper::body::Incoming>;

pub type BodyType = BoxBody<Bytes, hyper::Error>;

pub type ResponseType = Result<Response<BodyType>, Box<dyn std::error::Error + Send + Sync>>;

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
