use crate::service::transfer::FinishDetectableBody;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Request, Response};

pub type BodyType = BoxBody<Bytes, hyper::Error>;

pub type ResponseType = Result<Response<BodyType>, Box<dyn std::error::Error + Send + Sync>>;

pub type RequestType = Request<hyper::body::Incoming>;

pub type DataReceiverResponseBody = FinishDetectableBody<BodyType>;

pub type TransferResponseType =
    Result<Response<DataReceiverResponseBody>, Box<dyn std::error::Error + Send + Sync>>;
