use std::future::{Ready, ready};
use std::sync::Arc;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::http::header::HeaderMap;
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, web};

use crate::Context;

#[derive(Clone)]
pub struct RequestContext {
    ctx: Arc<Context>,
    headers: HeaderMap,
}

impl RequestContext {
    pub fn new(ctx: Arc<Context>, headers: HeaderMap) -> Self {
        Self { ctx, headers }
    }

    pub fn context(&self) -> &Context {
        &self.ctx
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }
}

impl FromRequest for RequestContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let request_ctx = req
            .extensions()
            .get::<RequestContext>()
            .cloned()
            .expect("RequestContext not found in request extensions");

        ready(Ok(request_ctx))
    }
}

impl std::ops::Deref for RequestContext {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        self.context()
    }
}

pub struct RequestContextMiddleware;

impl<S, B> Transform<S, ServiceRequest> for RequestContextMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestContextMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestContextMiddlewareService { service }))
    }
}

pub struct RequestContextMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestContextMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let ctx = req
            .app_data::<web::Data<Context>>()
            .expect("Context not found in app data")
            .clone()
            .into_inner();

        let headers = req.headers().clone();
        let request_ctx = RequestContext::new(ctx, headers);

        req.extensions_mut().insert(request_ctx);

        self.service.call(req)
    }
}
