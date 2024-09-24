use core::{cell::RefCell, fmt, future::Future, marker::PhantomData};

use std::net::SocketAddr;

use http_body::Body;
use xitca_http::{
    bytes::Bytes,
    h1::RequestBody,
    http::{Request, RequestExt, Response},
    HttpServiceBuilder,
};
use xitca_io::net::io_uring::TcpStream;
use xitca_service::{
    fn_build, middleware::UncheckedReady, ready::ReadyService, Service, ServiceExt,
};
use xitca_web::service::tower_http_compat::{CompatReqBody, CompatResBody};

pub struct TowerHttp<S, B> {
    service: RefCell<S>,
    _p: PhantomData<fn(B)>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

impl<S, B> TowerHttp<S, B> {
    pub fn service<F, Fut>(
        func: F,
    ) -> impl Service<
        Response = impl ReadyService + Service<(TcpStream, SocketAddr)>,
        Error = impl fmt::Debug,
    >
    where
        F: Fn() -> Fut + Send + Sync + Clone,
        Fut: Future<Output = Result<S, Error>>,
        S: tower::Service<
            Request<CompatReqBody<RequestExt<RequestBody>, ()>>,
            Response = Response<B>,
        >,
        S::Error: fmt::Debug,
        B: Body<Data = Bytes> + Send + 'static,
    {
        fn_build(move |_| {
            let func = func.clone();
            async move {
                func().await.map(|service| TowerHttp {
                    service: RefCell::new(service),
                    _p: PhantomData,
                })
            }
        })
        .enclosed(UncheckedReady)
        .enclosed(HttpServiceBuilder::h1().io_uring())
    }
}

impl<S, B> Service<Request<RequestExt<RequestBody>>> for TowerHttp<S, B>
where
    S: tower::Service<Request<CompatReqBody<RequestExt<RequestBody>, ()>>, Response = Response<B>>,
{
    type Response = Response<CompatResBody<B>>;
    type Error = S::Error;

    async fn call(
        &self,
        req: Request<RequestExt<RequestBody>>,
    ) -> Result<Self::Response, Self::Error> {
        let (parts, ext) = req.into_parts();
        let req = Request::from_parts(parts, CompatReqBody::new(ext, ()));
        let fut = self.service.borrow_mut().call(req);
        let (parts, body) = fut.await?.into_parts();
        Ok(Response::from_parts(parts, CompatResBody::new(body)))
    }
}
