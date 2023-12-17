use super::error::Invalid;
use pin_project_lite::pin_project;
use tower_http::BoxError;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pin_project! {
    #[derive(Debug)]
    pub struct ResponseFuture<T> {
        #[pin]
        response: T,
        #[pin]
        auth_type: AuthType,
    }
}

impl<T> ResponseFuture<T> {
    pub(crate) fn new(response: T, auth_type: AuthType) -> Self {
        ResponseFuture { response, auth_type }
    }
}

impl<F, T, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<T,E>>,
    E: Into<BoxError>,
{
    type Output = Result<T, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.reponse.poll(cx) {
            Poll
        }
    }
}
