use std::future::{ready, Ready};

use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey};
use r2d2_redis::redis::Commands;
use serde_json::json;

use crate::dao::redis_db::get_redis;
use crate::define::JWT_SECRET;
use crate::handler::user::UserClaim;
use crate::models::r#enum::redis_enum::RedisEnum;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        if token.is_none() {
            let response = HttpResponse::Unauthorized()
                .insert_header((header::CONTENT_TYPE, "application/json"))
                .json(json!({ "code": 401, "msg": "token is empty" }))
                .map_into_boxed_body();

            let (req, _) = req.into_parts();
            return Box::pin(async move { Ok(ServiceResponse::new(req, response)) });
        }

        let token = token.unwrap();

        let mut conn = match get_redis() {
            Ok(c) => c,
            Err(_) => {
                let response = HttpResponse::Unauthorized()
                    .insert_header((header::AUTHORIZATION, "application/json"))
                    .json(json!({ "code": 401, "msg": "redis unavailable" }))
                    .map_into_boxed_body();
                let (req, _) = req.into_parts();
                return Box::pin(async move { Ok(ServiceResponse::new(req, response)) });
            }
        };

        let redis_key = RedisEnum::LogInUser.to_key().to_string() + &token;
        let redis_val: Option<String> = conn.get(&redis_key).ok();
        if redis_val.is_none() {
            let response = HttpResponse::Unauthorized()
                .insert_header((header::AUTHORIZATION, "application/json"))
                .json(json!({ "code": 401, "msg": "token is invalid" }))
                .map_into_boxed_body();
            let (req, _) = req.into_parts();
            return Box::pin(async move { Ok(ServiceResponse::new(req, response)) });
        }

        let result = decode::<UserClaim>(
            &token,
            &DecodingKey::from_secret(JWT_SECRET.as_ref()),
            &jsonwebtoken::Validation::default(),
        );
        if result.is_err() {
            let response = HttpResponse::Unauthorized()
                .insert_header((header::AUTHORIZATION, "application/json"))
                .json(json!({ "code": 401, "msg": "token decode failed" }))
                .map_into_boxed_body();
            let (req, _) = req.into_parts();
            return Box::pin(async move { Ok(ServiceResponse::new(req, response)) });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?.map_into_boxed_body();
            Ok(res)
        })
    }
}
