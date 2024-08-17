use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Request, Response};

pub type ResponseType = Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>;

pub type RequestType = Request<hyper::body::Incoming>;
