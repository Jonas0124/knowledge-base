use crate::define::JWT_SECRET;
use crate::handler::user::UserClaim;
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use jsonwebtoken::{decode, DecodingKey};
use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};

// 定义用户上下文结构
#[derive(Debug, Clone)]
pub struct UserContext {
    pub id: String,

    pub username: String,

    pub email: String,
}

// 定义 Task Local
// std::thread_local! {
//     pub static USER_CONTEXT: RefCell<Option<UserContext>> = RefCell::new(None);
// }

// 自定义中间件
pub struct UserContextMiddleware;

impl<S, B> Transform<S, ServiceRequest> for UserContextMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = UserContextMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(UserContextMiddlewareService { service }))
    }
}

pub struct UserContextMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for UserContextMiddlewareService<S>
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

    fn call<'a>(&'a self, req: ServiceRequest) -> Self::Future {
        let mut user_context = UserContext {
            id: String::from("0"),
            username: String::from(""),
            email: "".to_string(),
        };

        let token = req.headers().get("Authorization");
        if token.is_some() {
            let token = token.unwrap().to_str().unwrap();
            let result = decode::<UserClaim>(token, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &jsonwebtoken::Validation::default());
            if let Ok(result) = result {
                let claim = result.claims;
                user_context.id = claim.id;
                user_context.username = claim.username;
                user_context.email = claim.email;
            }
        }
        req.extensions_mut().insert(user_context);

        let fut = self.service.call(req);

        Box::pin(fut)
    }
}

// 处理请求
// async fn index(req: HttpRequest) -> impl Responder {
//     // 从 Task Local 中获取用户上下文
//     if let Ok(ctx) = USER_CONTEXT.try_with(|ctx| ctx.clone()) {
//         format!("Hello, user_id: {}, role: {}", ctx.user_id, ctx.role)
//     } else {
//         "No user context available".to_string()
//     }
// }
