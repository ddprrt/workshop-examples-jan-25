use std::time::Instant;

use futures::future::BoxFuture;
use hyper::Request;
use tower::{Layer, Service};

#[derive(Clone, Copy)]
pub struct LogService<S> {
    inner: S,
}

impl<S> LogService<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> Service<Request<B>> for LogService<S>
where
    S: Service<Request<B>> + Clone + Send + 'static,
    S::Future: Send,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<S::Response, S::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let mut this = self.inner.clone();
        let method = req.method().to_owned();
        let path = req.uri().path().to_owned();
        let prev = Instant::now();
        println!("processing {} {}", method, path);
        Box::pin(async move {
            let res = this.call(req).await;
            let curr = Instant::now();
            println!(
                "end processing {} {} {}",
                curr.duration_since(prev).as_nanos(),
                method,
                path
            );
            res
        })
    }
}

#[derive(Clone, Copy)]
pub struct LogLayer;

impl LogLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LogService::new(inner)
    }
}
