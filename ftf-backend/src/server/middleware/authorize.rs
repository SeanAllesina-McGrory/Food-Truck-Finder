#[path = "./authorize/error.rs"]
pub mod error;
#[path = "./authorize/future.rs"]
pub mod future;
#[path = "./authorize/layer.rs"]
mod layer;

use self::future::ResponseFuture;
use std::task::{Context, Poll};
use tower_http::BoxError;
use tower_service::Service;

use crate::server::middleware::authorize::layer::AuthType;

#[derive(Debug, Clone)]
pub struct Authorize<T> {
    inner: T,
    auth_type: AuthType,
}

impl<T> Authorize<T> {
    pub fn new(inner: T, auth_type: AuthType) -> Self {
        Authorize { inner, auth_type }
    }

    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<S, Request> Service<Request> for Authorize<S>
where
    S: Service<Request>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match self.inner.poll_ready(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(r) => Poll::Ready(r.map_err(Into::into)),
        }
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let response = self.inner.call(request);
        let auth = true;

        ResponseFuture::new(response, auth)
    }
}
