use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::HeaderName;
use actix_web::{web, Error, HttpMessage};
use futures::executor::block_on;
use futures_util::{FutureExt, StreamExt};
use std::future::{ready, Future, Ready};
use std::io::Read;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::task::block_in_place;
use tracing::{info, span, Instrument, Level};
use uuid::Uuid;
use uuid::Version::Custom;

// 定义用户上下文结构

// 自定义中间件
pub struct LogMiddleware;

impl<S, B> Transform<S, ServiceRequest> for LogMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LogMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LogMiddlewareService { service }))
    }
}

pub struct LogMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LogMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {

        let trace_id = Uuid::new_v4().to_string();
        tracing::info!(trace_id = %trace_id, "New request received");
        // 将 Trace ID 添加到请求头部
        req.headers_mut().insert(
            HeaderName::from_bytes("x-trace-id".as_bytes()).unwrap(),
            actix_web::http::header::HeaderValue::from_str(&trace_id).unwrap(),
        );
        // Create a span with the trace_id
        let span = span!(Level::INFO, "", trace_id = %trace_id);


        // Log the trace_id
        tracing::info!("Processing request: {}", req.path());
        // 调用服务处理请求.instrument(span)
        let fut = self.service.call(req);

        Box::pin(async move {
            let response = fut.instrument(span).await?;
            tracing::info!("trace_id={}:=Processing requestsssss: {}",&trace_id, response.status());
            Ok(response)
        })
    }
}

