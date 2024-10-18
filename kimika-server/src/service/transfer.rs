use crate::data;
use http_body_util::{combinators::BoxBody, BodyExt};
use pin_project_lite::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pin_project! {
    pub struct FinishDetectableBody<B> {
        #[pin]
        body: B,
        finish_notifier: Option<tokio::sync::oneshot::Sender<()>>,
    }
}

impl<B: http_body::Body> http_body::Body for FinishDetectableBody<B> {
    type Data = B::Data;
    type Error = B::Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<http_body::Frame<Self::Data>, Self::Error>>> {
        let mut this = self.project();
        match this.body.as_mut().poll_frame(cx) {
            // If body is finished
            Poll::Ready(None) => {
                // Notify finish
                if let Some(notifier) = this.finish_notifier.take() {
                    notifier.send(()).unwrap();
                }
                Poll::Ready(None)
            }
            poll => poll,
        }
    }

    #[inline]
    fn is_end_stream(&self) -> bool {
        self.body.is_end_stream()
    }

    #[inline]
    fn size_hint(&self) -> http_body::SizeHint {
        self.body.size_hint()
    }
}

#[derive(Debug)]
struct TransferError {
    message: String,
}

impl TransferError {
    fn new(message: String) -> Self {
        TransferError { message }
    }
}

impl std::fmt::Display for TransferError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TransferError {}

pub async fn transfer(
    sender: data::DataSender,
    receiver: data::DataReceiver,
) -> Result<(), Box<dyn std::error::Error>> {
    let data::DataSender {
        req_body,
        res_body_tx: sender_res_body_tx,
    } = sender;

    let body = FinishDetectableBody {
        body: req_body.boxed(),
        finish_notifier: Some(sender_res_body_tx),
    };

    let body = BoxBody::new(body);

    let res = hyper::Response::builder().body(body).unwrap();
    receiver
        .res_sender
        .send(res)
        .map_err(|e| TransferError::new(format!("Transfer error: {:?}", e.headers())))?;

    Ok(())
}
