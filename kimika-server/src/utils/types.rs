use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Request, Response};

pub type ResponseType =
    Result<Response<BoxBody<Bytes, hyper::Error>>, Box<dyn std::error::Error + Send + Sync>>;

pub type RequestType = Request<hyper::body::Incoming>;
